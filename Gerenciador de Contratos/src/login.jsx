import { useState } from "react";
import { useNavigate } from 'react-router-dom';

function Login(){
    const [mensagemEmail, setMensagemEmail] = useState("");
    const [email, setEmail] = useState("");
    localStorage.removeItem('token');
    const [mensagemSenha, setMensagemSenha] = useState("");
    const [senha, setSenha] = useState("");
  
    async function checaEmail() {
      fetch('http://localhost:3000/checa_email', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        body: JSON.stringify(email)
      })
      .then(response => {
        if (!response.ok) {
          return response.text().then(errorMessage => {
            setMensagemEmail(errorMessage);
            throw new Error(`Erro: ${errorMessage}`);
          });
        }
        console.log(response);
      })
      .catch(error => console.error(error));
    }
  
    async function verificaToken() {
      try {
        const token = localStorage.getItem('token');
        console.log('Token na verificação:', typeof token, token);
    
        const response = await fetch('http://localhost:3000/verifica_token', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
          },
          body: JSON.stringify({ email, token }),
        });
    
        if (!response.ok) {
          const errorMessage = await response.text();
          setMensagemSenha(errorMessage);
          throw new Error(`Erro: ${errorMessage}`);
        }
    
        console.log('Token válido!');
        home();
      } catch (error) {
        console.error('Erro ao verificar o token:', error);
      }
    }
  
    async function realizaLogin() {
      try {
        const response = await fetch('http://localhost:3000/verifica_senha', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify({ email, senha })
        });
    
        if (!response.ok) {
          const errorMessage = await response.text();
          setMensagemSenha(errorMessage);
          throw new Error(`Erro: ${errorMessage}`);
        }
    
        console.log('Resposta de verifica_senha:', response);
    
        const buscaIdResponse = await fetch('http://localhost:3000/busca_id', {
          method: 'POST',
          headers: {
            'Content-Type': 'application/json'
          },
          body: JSON.stringify(email)
        });
        
    
        if (!buscaIdResponse.ok) {
          const errorMessage = await buscaIdResponse.text();
          setMensagemSenha(errorMessage);
          throw new Error(`Erro: ${errorMessage}`);
        }
    
        const novo_token = await buscaIdResponse.text();
        localStorage.setItem('token', novo_token);
        console.log("Token gerado ao entrar: ", novo_token);
    
        if (localStorage.getItem('token')) {
          console.log("Token foi definido.");
          home();
        }
    
        setMensagemSenha('Entrando na conta!');
    
      } catch (error) {
        console.error('Erro no login:', error);
      }
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
            await checaEmail();
            await realizaLogin();          
            await verificaToken();
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