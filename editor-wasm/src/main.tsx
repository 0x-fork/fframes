// import React from "react";
// import ReactDOM from "react-dom";
// import "./index.css";
// import App from "./App";

// ReactDOM.createRoot(document.getElementById("root")).render(
//   <React.StrictMode>
//     <App />
//   </React.StrictMode>
// );

import * as videoWasmBinding from "../bind/pkg";
import { renderEditor } from "fframes-editor";
import "fframes-editor/tw.css";

console.log(videoWasmBinding);

renderEditor(import.meta.globEager("../media/*"), videoWasmBinding);
