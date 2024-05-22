import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
//npmimport "./App.css";



function ResetSenha(){
  const [mensagemReset, setMensagemReset] = useState("");
  const [email, setEmail] = useState("");

  async function loginEmail() {
    const r = await invoke("encontra_email", { email });
    if (r){
      setMensagemReset("Email enviado");
    } else{
      setMensagemReset("Email não enviado");
    }
    console.log(r);
  }
  

    return (
      <div className="formReset">

        <form
          className="rowReset"
          onSubmit={(e) => {
            e.preventDefault();
            loginEmail();
          }}
        >
          <input required
            id="email-input"
            onChange={(e) => setEmail(e.currentTarget.value)}
            placeholder="Seu email..." 
          />
        <p id="mensagemReset">{mensagemReset}</p>
        <button type="submit">Enviar</button>
        <br />
        <button className="botao" type="button" onClick={() => window.location.href = "App.jsx"}>voltar</button>
        </form>
      </div>
    );
  }

  export default ResetSenha;