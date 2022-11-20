import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from '@tauri-apps/api/event'
import "./App.css";

function App() {

  const [cpuUsage, setcpuUsage] = useState()
  const [totalMemory, setTotalMemory] = useState()
  const [memoryUsage, setMemoryUsage] = useState()

  useEffect(() => {
    get_total_memory()
    let unlisten: any;
    async function f() {
      unlisten = await listen('cpuusage', (event: any) => {
        setcpuUsage(event.payload)
      });
      unlisten = await listen('memoryusage', (event: any) => {
        setMemoryUsage(event.payload)
      });
    }
    f();

    return () => {
      if (unlisten) {
        unlisten();
      }
    }
  }, [])

  async function get_total_memory() {
    setTotalMemory(await invoke("total_memory"))
  }


  return (
    <div className="shadow">
      <div data-tauri-drag-region className="container">
        <div data-tauri-drag-region className="status">
          <span className="material-symbols-rounded">
          memory
          </span>
          <progress className="memory" value={cpuUsage} max={100}></progress>
        </div>
        <div data-tauri-drag-region className="status">
          <span className="material-symbols-rounded">
          storage
          </span> 
          <progress className="memory" value={memoryUsage} max={totalMemory}></progress>
        </div>
      </div>
    </div>
  );
}

export default App;
