// import React from "react";
// import ReactDOM from "react-dom";
// import "./index.css";
// import App from "./App";

// ReactDOM.createRoot(document.getElementById("root")).render(
//   <React.StrictMode>
//     <App />
//   </React.StrictMode>
// );

import * as videoWasmBinding from "./editor-bridge/pkg/editor-bridge";
import { renderEditor } from "fframes-editor";
import "fframes-editor/tw.css";
console.log(
  import.meta.glob("../media/*", {
    as: "url",
    eager: true,
  })
);
renderEditor(
  import.meta.glob("../media/*", {
    as: "url",
    eager: true,
  }),
  videoWasmBinding
);
