import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";


function Login(){
  const [mensagemEmail, setMensagemEmail] = useState("");
  const [email, setEmail] = useState("");

  const [mensagemSenha, setMensagemSenha] = useState("");
  const [senha, setSenha] = useState("");

  async function loginEmail() {
    setMensagemEmail(await invoke("login_email", { email }));
  }

  async function loginSenha(){
    const retorno_conta_encontrada = await invoke("login_senha", {email, senha});
    if(retorno_conta_encontrada){
      setMensagemSenha("Entrando na conta!");
      window.location.href = "menu.html";
    } 
    if (!retorno_conta_encontrada){
      setMensagemSenha("Login mal-sucedido");
    }
  }
  
  return (
    <div id="camposLoginForm">
       <p className="subtitulo">conecte-se</p>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          loginEmail();
          loginSenha();
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
      
       <button className="resetSenha" type="button" onClick={() => window.location.href = "reseta_senha.html"}>Esqueci a senha</button>
        
       <button id="botaoCriarContaForm" type="button"onClick={()=> window.location.href= "criar_conta.html"}>Criar conta</button>
    
      </form>
        
      
    </div>
    
  );
}

export default Login;