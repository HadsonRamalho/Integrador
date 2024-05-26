import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";


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
      <div>
        <form
          onSubmit={(e) => {
            e.preventDefault();
            loginEmail();
          }}
        >
          <input required
          
            className="rowReset"
            onChange={(e) => setEmail(e.currentTarget.value)}
            placeholder="Seu email..." 
          />
        <p className="mensagemLogin">{mensagemReset}</p>
        <button type="submit" >Enviar</button>
        <br />
        <button className="botaovoltar" type="button" onClick={() => window.location.href = "App.jsx"}>voltar</button>
        </form>
      </div>
    );
  }

  export default ResetSenha;