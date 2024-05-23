import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
function App() {
  useEffect(() => {
    invoke("set_always_on_top");
    window.location.href = 'https://translate.google.com/';
  }, [])

  return (
    <div className="container">
 

    </div>
  );
}

export default App;
