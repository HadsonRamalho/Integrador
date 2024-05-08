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
    const [mensagem, sucesso] = retorno;
    setMensagemCriarConta(mensagem);
    console.log(mensagem);
    //DB // [DEV | BACK] : Tratar criação duplicada
    if (sucesso){
      try {
        //await invoke("email_repetido", {email});
        await invoke("save_data", {email});
        console.log('Dados salvos com sucesso no banco de dados!');
      } catch (error) {
          console.error('Erro ao salvar dados:', error);
      }
    }   

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