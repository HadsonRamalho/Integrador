import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    setGreetMsg(await invoke("greet", { name }));
  }
  async function greet2(){
    setGreetMsg(await invoke("greet2", {name}));
  }
  
  return (
    <div className="container">
      <p>[DEV | Back] Testando busca de email</p>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          greet2();
        }}
      >
        <input
          id="greet-input"
          onChange={(e) => setName(e.currentTarget.value)}
          placeholder="E-mail a ser procurado..."
        />
        <button type="submit">Procurar</button>
      </form>

      <p>{greetMsg}</p>
    </div>
  );
}

function SignUp(){
  return (
    <div className="formLogin">
      <p>TESTE</p>
      <form
        className="row"
      >
        <input required
          id="email-input"
          placeholder="Novo email..." 
        />  
        <input required
          id="senha-input"
          placeholder="Nova senha..."
        />
      <p id="mensagemLogin">  </p>
      <button type="submit">Procurar</button>
      </form>
    </div>
  );
}

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
      <p id="mensagemLogin"> {mensagemEmail} <br></br >{mensagemSenha} </p>
      <button type="submit">Procurar</button>
      </form>
    </div>
  );
}



export default Login;
