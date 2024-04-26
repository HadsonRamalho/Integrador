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
    setMensagemEmail(await invoke("login_email", { email }));
  }

  async function loginSenha(){
    const retorno = await invoke("login_senha", {email, senha});
    const [mensagem, sucesso] = retorno;
    setMensagemSenha(mensagem);
    console.log(sucesso);
  }
  
  return (
    <div className="formLogin">
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
          placeholder="E-mail " 
        />  
        <input required
          id="senha-input"
          onChange={(e) => setSenha(e.currentTarget.value)}
          placeholder="Senha"
          type="password"
        />
      <p id="mensagemLogin"> {mensagemEmail} <br></br >{mensagemSenha} </p>
  
      <button type="submit">Entrar</button>
      </form>
    </div>
  );
}

export default Login;