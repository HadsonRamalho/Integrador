import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function SignUp(){

  const [mensagemCriarConta, setMensagemCriarConta] = useState("");
  const [email, setEmail] = useState("");

  const [nomeCompleto, setNomeCompleto] = useState("");

  const [senha1, setSenha1] = useState("");
  
  const [senha2, setSenha2] = useState("");

  async function criarConta() {
    const retorno = await invoke("cria_conta", { email, nomeCompleto, senha1, senha2});
    const mensagem = retorno;
    setMensagemCriarConta(mensagem);
    console.log(mensagem);
  }


    return (
      <div className="formSignUp">
        <p>[DEV | BACK] | Testando criação de conta</p>
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
            placeholder="Nome completo..."
            />
          <input required
            id="email-input"
            onChange={(e) => setEmail(e.currentTarget.value)}
            placeholder="E-mail " 
          />
          <input required
            id="senha-input"
            onChange={(e) => setSenha1(e.currentTarget.value)}
            placeholder="Sua senha..."
          />
          <input required
            id="senha-input"
            onChange={(e) => setSenha2(e.currentTarget.value)}
            placeholder="Confirme sua senha..."
          />
          <p id="mensagemLogin"> {mensagemCriarConta} </p>
        <button type="submit">Criar</button>
    
        </form>
      </div>
    );
  }

  export default SignUp;