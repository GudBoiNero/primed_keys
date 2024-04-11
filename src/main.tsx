import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.scss";
import { invoke } from "@tauri-apps/api/tauri";

invoke('init').then(() => {
  console.log("Initialized Backend!")
}).catch(err => {
  console.error(err)
})

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
