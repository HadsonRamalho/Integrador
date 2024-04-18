import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function SignUp(){
    return (
      <div className="formSignUp">
        <p>[DEV | BACK] | Testando criação de conta</p>
        <form
          className="rowSignUp"
        >
          <input required
            id="nome-input"
            placeholder="Nome completo..."
            />
          <input required
            id="email-input"
            placeholder="Seu email..." 
          />  
          <input required
            id="senha-input"
            placeholder="Sua senha..."
          />
          <input required
            id="senha-input"
            placeholder="Confirme sua senha..."
          />
        <p id="mensagemSignUp">  </p>
        <button type="submit">Criar</button>
        </form>
      </div>
    );
  }

  export default SignUp;