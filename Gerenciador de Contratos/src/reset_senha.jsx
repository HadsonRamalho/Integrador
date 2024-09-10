import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";


function ResetSenha(){
  const [mensagemReset, setMensagemReset] = useState("");
  const [email, setEmail] = useState("");

  async function loginEmail() {
    try{
      const codigo = await invoke("encontra_email_smtp", { email });
      localStorage.setItem('codigoReset', codigo);
    } catch(error){
      setMensagemReset(error);
      console.log(error);
    }
  }

  const navigate = useNavigate();

  const login = () => {
    navigate('/');
  };

  const redefinicao_senha = () => {
    navigate ('/redefinicao_senha');
  };
  
    return (
      <div id="boxreset">
        <div>
        <p className="subtitulo">redefinir senha</p>
        </div>
        <form
          onSubmit={async (e) => {
            e.preventDefault();
            await loginEmail();
            redefinicao_senha();
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
        <button className="botaovoltar" type="button" onClick={login}>voltar</button>
        </form>
      </div>
    );
  }

  export default ResetSenha;