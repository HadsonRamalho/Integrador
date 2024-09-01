import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";
//import "./App.css";


function CriaConta(){

  const [mensagemCriarConta, setMensagemCriarConta] = useState("");
  const [email, setEmail] = useState("");

  const [nomeCompleto, setNomeCompleto] = useState("");

  const [senha1, setSenha1] = useState("");
  
  const [senha2, setSenha2] = useState("");

  async function criarConta() {
    try{
      await invoke("cria_conta", {nomeCompleto, email, senha1, senha2});
      setMensagemCriarConta("Conta criada");
    }
    catch(error){
      setMensagemCriarConta(error);
    }
  }

  const navigate = useNavigate();

  const login = () => {
    navigate('/');
  };

    return (
      <div id="boxcriar">
        <p id="subtituloForm"></p>
        <div>
        <p className="subtitulo">Cadastre-se</p>
        </div>
        <form
          className="rowSignUp"
          onSubmit={(e) => {
            e.preventDefault();
            criarConta();
          }}
        >
          <input required
            className="user-input"
            onChange={(e) => setNomeCompleto(e.currentTarget.value)}
            placeholder="Nome completo"
            />
          <input required
            className="user-input"
            onChange={(e) => setEmail(e.currentTarget.value)}
            placeholder="E-mail " 
          />
          <input required
            className="user-input"
            
            onChange={(e) => setSenha1(e.currentTarget.value)}
            placeholder="Sua senha"
            type="password"
          />
          <input required
            className="user-input"
            onChange={(e) => setSenha2(e.currentTarget.value)}
            placeholder="Confirme sua senha"
            type="password"
          />
          <p className="mensagemLogin"> {mensagemCriarConta} </p>  
        <button className="user-input" type="submit">Criar</button>
        <div>
          <br />
          <button className="botaovoltar" type="button" onClick={login}>Ja tenho conta</button>
        </div>
        </form>
      </div>
    );
  }

  export default CriaConta;