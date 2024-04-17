import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("camposLoginForm")).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

document.getElementById("botaoCriarContaForm").addEventListener("click", function() {
  window.location.href = "criar_conta.html";
});