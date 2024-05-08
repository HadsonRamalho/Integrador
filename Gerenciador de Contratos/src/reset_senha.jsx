import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
//npmimport "./App.css";



function ResetSenha(){
    const [mensagemReset, buscaEmail] = useState("");
    const [email, setEmai] = useState("");
  
    async function buscarEmail() {
      buscaEmail(await invoke("buscaEmail", { email }));
    }

    return (
      <div className="formReset">

        <form
          className="rowReset"
          onSubmit={(e) => {
            e.preventDefault();
            buscarEmail();
          }}
        >
          <input required
            id="email-input"
            onChange={(e) => setEmai(e.currentTarget.value)}
            placeholder="Seu email..." 
          />
        <p id="mensagemReset">{mensagemReset}</p>
        <button type="submit">Enviar</button>
        </form>
      </div>
    );
  }

  export default ResetSenha;