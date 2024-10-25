import { useState } from "react";
import { useNavigate } from "react-router-dom";


function ResetSenha(){
  const [mensagemReset, setMensagemReset] = useState("");
  const [email, setEmail] = useState("");

  async function loginEmail() {
    try {  
      const response = await fetch('http://localhost:3000/encontra_email_smtp', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify({email}),
      });
  
      if (!response.ok) {
        const errorMessage = await response.text();
        setMensagemReset(errorMessage);
        throw new Error(`Erro: ${errorMessage}`);
      }

      const codigo =  await response.json();
      console.log("Codigo:", codigo);
      localStorage.setItem('codigoReset', codigo);
      localStorage.setItem('emailReset', email);
      redefinicao_senha();
    } catch (error) {
      setMensagemReset(error);
      console.error('Erro ao verificar o token:', error);

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