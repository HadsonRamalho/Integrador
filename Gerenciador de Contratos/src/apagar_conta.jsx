import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

function ApagarConta(){
  const [mensagem, setMensagem] = useState("");
  const [senha, setSenha] = useState("");

  async function apagaConta() {
    try{
      const id = localStorage.getItem('token');
      const idusuario = id;
      const email = await invoke("busca_email_usuario", { id });
      await invoke("verifica_senha", {email, senha});
      await invoke("deleta_conta", {idusuario, email});
      setMensagem("Saindo da conta!");
      login();
    } catch(error){
      setMensagem(error);
      console.log("[Apagar_conta.jsx] : ", error);
    }
  }

  const navigate = useNavigate();

  const login = () => {
    navigate('/');
  };

  const home = () => {
    navigate('/home');
  };

    return (
      <div id="boxreset">
        <div>
        <p className="subtitulo">apagar minha conta</p>
        </div>
        <form
          onSubmit={async (e) => {
            e.preventDefault();
            await apagaConta();
          }}
        >
          <input required
          className="rowReset"
          onChange={(e) => setSenha(e.currentTarget.value)}
          placeholder="Confirme sua senha." 
          type="password"
        />        
        <p className="mensagemLogin">{mensagem}</p>        
        <button type="submit" >Apagar minha conta</button>
        <br />
        <button onClick={home} >Voltar</button>
        </form>
      </div>
    );
  }

  export default ApagarConta;