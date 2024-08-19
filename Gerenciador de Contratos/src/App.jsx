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

  async function estruturaLocadora(idendereco){
    const cnpj = "123456";
    const numerocontabanco = "21345";
    const numeroagenciabanco = "2123";
    const nomebanco = "Banco Ruim";
    const nomelocadora = "DesLocadora";
    const locadora = await invoke("estrutura_locadora", {idendereco, cnpj, numerocontabanco, numeroagenciabanco, nomebanco, nomelocadora});
    return locadora;
  }

  async function cadastraLocadora(idendereco){
    try{
      const locadora = await estruturaLocadora(idendereco);
      console.log(locadora);
      await invoke("cadastra_locadora", {locadora});
    } catch (error){
      console.log(error);
    }
  }

  async function estruturaEndereco(){
    const logradouro = 'Rua das Ruas';
    const cep = '12345-678';
    const complemento = 'Complemento Tal';
    const numeroendereco = '123';
    const cidade = 'Cidade das Cidades';
    const uf = 'NO';
    const endereco = await invoke("estrutura_endereco", {
          logradouro, 
          cep, 
          complemento, 
          numeroendereco, 
          cidade, 
          uf
      });
    return endereco;
  }

  async function cadastraEndereco(){
    const endereco = await estruturaEndereco();
    try{
      const idendereco = await invoke("_salva_endereco", {endereco});
      return idendereco;
    } catch(error){
      console.log('Erro ao salvar o endereço: ', error);
    }
  }

  async function loginSenha(){
    try{
      const retorno_conta_encontrada = await invoke("login_senha", {email, senha});
    } catch (error){
      setMensagemSenha(error); // Alterar para mensagem de erro personalizada
      return;
    }
    setMensagemSenha("Entrando na conta!");    
    const novo_token = await invoke("gera_token", {email}); //Preparando autenticação
    localStorage.setItem('token', novo_token); // Armazenando token
  }
  
  return (
    <div id="camposLoginForm">
       <p className="subtitulo">conecte-se</p>
      <form
        className="row"
        onSubmit={async(e) => {
          e.preventDefault();
          checaEmail();
          loginSenha();
          const idendereco = await cadastraEndereco();
          cadastraLocadora(idendereco);
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