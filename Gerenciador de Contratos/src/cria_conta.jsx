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
            id="email-input"
            placeholder="Novo email..." 
          />  
          <input required
            id="senha-input"
            placeholder="Nova senha..."
          />
        <p id="mensagemSignUp">  </p>
        <button type="submit">Criar</button>
        </form>
      </div>
    );
  }

  export default SignUp;