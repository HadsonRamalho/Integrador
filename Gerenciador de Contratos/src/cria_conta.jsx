import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
//import "./App.css";

function SignUp(){

  const [mensagemCriarConta, setMensagemCriarConta] = useState("");
  const [email, setEmail] = useState("");

  const [nomeCompleto, setNomeCompleto] = useState("");

  const [senha1, setSenha1] = useState("");
  
  const [senha2, setSenha2] = useState("");

  async function criarConta() {
    const retorno_conta_criada = await invoke("cria_conta", {nomeCompleto, email, senha1, senha2});
    if (retorno_conta_criada){
      setMensagemCriarConta("Conta criada");
    } else{
      setMensagemCriarConta("Conta não foi criada");
    }
    console.log(retorno);
    //DB // [DEV | BACK] : Tratar criação duplicada

  }


    return (
      <div className="formSignUp">
        <form
          className="rowSignUp"
          onSubmit={(e) => {
            e.preventDefault();
            criarConta();
          }}
        >
          <input required
            id="nome-input"
            onChange={(e) => setNomeCompleto(e.currentTarget.value)}
            placeholder="Nome completo"
            />
          <input required
            id="email-input"
            onChange={(e) => setEmail(e.currentTarget.value)}
            placeholder="E-mail " 
          />
          <input required
            id="senha-input"
            onChange={(e) => setSenha1(e.currentTarget.value)}
            placeholder="Sua senha"
            type="password"
          />
          <input required
            id="senha-input"
            onChange={(e) => setSenha2(e.currentTarget.value)}
            placeholder="Confirme sua senha"
            type="password"
          />
          <p id="mensagemLogin"> {mensagemCriarConta} </p>
        <button type="submit">Criar</button>
    
        </form>
      </div>
    );
  }

  export default SignUp;