import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import Nav from "./nav";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Nav />
    <App />
  </React.StrictMode>,
);
