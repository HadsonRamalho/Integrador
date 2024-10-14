import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from 'react-router-dom';

function Login(){
    const [mensagemEmail, setMensagemEmail] = useState("");
    const [email, setEmail] = useState("");
    localStorage.removeItem('token');
    const [mensagemSenha, setMensagemSenha] = useState("");
    const [senha, setSenha] = useState("");
  
    async function checaEmail() {
      setMensagemEmail(await invoke("checa_email", { email }));
    }
  
    async function verificaToken(){
      try{
        const token = localStorage.getItem('token');
        console.log('Token na verificação:', typeof token, token);
        const validatoken = await invoke("verifica_token", {email, token});
        console.log(validatoken);
        home();
      } catch(error){
        console.log("[Login.jsx | verificaToken] : ", error);
      }
    }
  
    async function realizaLogin(){
      fetch('http://localhost:3000/verifica_senha', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json' // Define o tipo de conteúdo
        },
        body: JSON.stringify({ email, senha }) // Converte os dados em JSON
      })
      .then(response => {
        if (!response.ok) {
          return response.text().then(errorMessage => {
            setMensagemSenha(errorMessage);
            throw new Error(`Erro do servidor: ${errorMessage}`);
          });
        }
        console.log(response);
      })
      .then(_ => console.log('Sucesso no login.'))
      .catch(error => console.error(error)); 
      setMensagemSenha('Entrando na conta!');
    }
    
  const navigate = useNavigate();

  const home = () => {
    navigate('/home');
  };

  const reset_senha = () =>{
    navigate('/reset_senha');
  };

  const cria_conta = () => {
    navigate('/cria_conta');
  }; 

    return (
    <div id="box">
      <div id="camposLoginForm">
         <p className="subtitulo">conecte-se</p>
        <form
          className="row"
          onSubmit={async(e) => {
            e.preventDefault();
            //await checaEmail();
            await realizaLogin();          
            //await verificaToken();
          }}
        >
          <input required
            onChange={(e) => setEmail(e.currentTarget.value)}
            placeholder="E-mail " 
          />  
          <input required
            onChange={(e) => setSenha(e.currentTarget.value)}
            placeholder="Senha"
            type="password"
          />
        <p className="mensagemLogin"> {mensagemEmail} <br></br >{mensagemSenha} </p>
    
        <button className="row"
         type="submit">Entrar</button>
        
        <button className="resetSenha" type="button" onClick={reset_senha}>Esqueci a senha</button>
      
         <button id="botaoCriarContaForm" type="button" onClick={cria_conta}>Criar conta</button>
      
        </form>          
        
      </div>
      </div>
      
    );
  }
  
  export default Login;