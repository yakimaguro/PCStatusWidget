#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayEvent, SystemTrayMenuItem};
use tauri::Manager;
use sysinfo::{CpuExt, System, SystemExt};

#[tauri::command]
fn total_memory() -> u64 {
    let mut sys = System::new_all();
    sys.refresh_all();
    return sys.total_memory()
}

#[tauri::command]
fn memory_usage() -> u64 {
    let mut sys = System::new_all();
    sys.refresh_all();
    return sys.used_memory()
}

#[tauri::command]
fn cpu_usage() -> f32 {
    let mut sys = System::new_all();
    sys.refresh_cpu();
    let processor = sys.global_cpu_info();
    return processor.cpu_usage();
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
    .add_item(quit);

    tauri::Builder::default()
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            _ => {}
            }
        }
        _ => {}
        })
        .invoke_handler(tauri::generate_handler![total_memory])
        .setup(|app| {
            let app_handle = app.app_handle();
            std::thread::spawn(move || loop {
                app_handle
                    .emit_all("cpuusage", cpu_usage())
                    .unwrap();
                app_handle
                    .emit_all("memoryusage", memory_usage())
                    .unwrap();    
                std::thread::sleep(std::time::Duration::from_secs(1))
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
