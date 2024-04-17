import { invoke } from "@tauri-apps/api";
import "./App.scss";

function App() {

  const macroUndo = () => {
    invoke("run_macro", { name: "undo" }).then(result => {
      console.log(`run_macro: ${result}`)
    }).catch(() => {
      console.log("run_macro: failed!")
    })
  }

  return (
    <>
      <div><button onClick={macroUndo} style={{ width: "200px", height: "200px" }}>Undo</button></div>
    </>
  );
}

export default App;
