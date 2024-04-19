import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function Login(){
  const [mensagemEmail, setMensagemEmail] = useState("");
  const [email, setEmail] = useState("");

  const [mensagemSenha, setMensagemSenha] = useState("");
  const [senha, setSenha] = useState("");

  async function loginEmail() {
    setMensagemEmail(await invoke("loginEmail", { email }));
  }

  async function loginSenha(){
    setMensagemSenha(await invoke("loginSenha", {email, senha}));
  }
  
  return (
    <div className="formLogin">
      <p>[DEV | Back] Testando email e senha</p>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          loginEmail();
          loginSenha();
        }}
      >
        <input required
          id="email-input"
          onChange={(e) => setEmail(e.currentTarget.value)}
          placeholder="E-mail a ser procurado..." 
        />  
        <input required
          id="senha-input"
          onChange={(e) => setSenha(e.currentTarget.value)}
          placeholder="Senha a ser procurada..."
        />
        <button type="button" id="resetSenha">Resetar senha</button>
      <p id="mensagemLogin"> {mensagemEmail} <br></br >{mensagemSenha} </p>
      <button type="submit">Procurar</button>
      </form>
    </div>
  );
}

export default Login;