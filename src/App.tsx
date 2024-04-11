import { invoke } from "@tauri-apps/api";
import "./App.scss";

function App() {

  const macroUndo = () => {
    invoke("macro_undo").then(result => {
      console.log("macro_undo: successfully invoked. Did it actually undo?")
      console.log(`macro_undo: ${result}`)
    }).catch(() => {
      console.log("macro_undo: failed to invoke!")
    })
  }

  return (
    <>
      <div><button onClick={macroUndo} style={{ width: "200px", height: "200px" }}>Undo</button></div>
    </>
  );
}

export default App;
