import React from "react";
import ReactDOM from "react-dom/client";
import Login from "./login";
import "./styles.css";

localStorage.removeItem('token');

ReactDOM.createRoot(document.getElementById("camposLoginForm")).render(
    <Login />
);

document.getElementById("botaoCriarContaForm").addEventListener("click", function() {
    window.location.href = "criar_conta.html";
  });
  
  document.getElementById("resetSenha").addEventListener("click", function() {
    window.location.href = "reseta_senha.html";
  });
  
  document.getElementById("menu").addEventListener("click", function() {
    window.location.href = "menu.html";
  });
  
  