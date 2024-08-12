import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";


function Login(){
  const [mensagemEmail, setMensagemEmail] = useState("");
  const [email, setEmail] = useState("");

  const [mensagemSenha, setMensagemSenha] = useState("");
  const [senha, setSenha] = useState("");

  async function checaEmail() {
    setMensagemEmail(await invoke("checa_email", { email }));
  }
  if (localStorage.getItem('token')){ // Se tiver um token definido, faz login direto no menu
    window.location.href = "menu.html";
  }

  async function loginSenha(){
    try{
      const retorno_conta_encontrada = await invoke("login_senha", {email, senha});
    } catch (error){
      setMensagemSenha(error);
      return;
    }
    setMensagemSenha("Entrando na conta!");
    const logradouro = 'Rua das Ruas';
    const cep = '12345-678';
    const complemento = 'Complemento Tal';
    const numeroendereco = '123';
    const cidade = 'Cidade das Cidades';
    const uf = 'NO';
    try {
      const endereco = await invoke("estrutura_endereco", {
          logradouro, 
          cep, 
          complemento, 
          numeroendereco, 
          cidade, 
          uf
      });      
      const salva = await invoke("_salva_endereco", {endereco});
      console.log('Endereço salvo com sucesso:', salva);
  } catch (error) {
      console.error('Erro ao salvar o endereço:', error);
  }
    const novo_token = await invoke("gera_token", {email}); //Preparando autenticação
    localStorage.setItem('token', novo_token); // Armazenando token
    window.location.href = "menu.html";
  }
  
  return (
    <div id="camposLoginForm">
       <p className="subtitulo">conecte-se</p>
      <form
        className="row"
        onSubmit={(e) => {
          e.preventDefault();
          checaEmail();
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