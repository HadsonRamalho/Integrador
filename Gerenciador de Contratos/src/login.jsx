import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import Home from "./home";
import { useNavigate } from 'react-router-dom';
import CriaConta from "./cria_conta";




function Login(){
    const [mensagemEmail, setMensagemEmail] = useState("");
    const [email, setEmail] = useState("");
    localStorage.removeItem('token');
    const [mensagemSenha, setMensagemSenha] = useState("");
    const [senha, setSenha] = useState("");
  
    async function checaEmail() {
      setMensagemEmail(await invoke("checa_email", { email }));
    }
  
    async function estruturaLocadora(idendereco){
      const cnpj = "123456";
      const numerocontabanco = "21345";
      const numeroagenciabanco = "2123";
      const nomebanco = "Banco Ruim";
      const nomelocadora = "DesLocadora";
      try{
        const locadora = await invoke("estrutura_locadora", {idendereco, cnpj, numerocontabanco, numeroagenciabanco, nomebanco, nomelocadora});
      }
      catch(error){
        console.log(error);
      }
      finally{
        return locadora;
      }    
    }
  
    async function cadastraLocadora(idendereco){
      try{
        const locadora = await estruturaLocadora(idendereco);
        await invoke("cadastra_locadora", {locadora});
      } catch (error){
        console.log(error);
      }
    }
      
    async function atualizaEmail(){
      const email = "user1000@u.com";
      try{
        await invoke("atualiza_email", {email});
      } catch(error){
        console.log(error);
      }
    }
    
    async function atualizaSenha(){
      try{
        const novaSenha = "novasenha";
        await invoke("atualiza_senha", {email, novaSenha});
      } catch (error){
        console.log(error);
      }
    }     
  
    async function verificaToken(){
      try{
        const token = localStorage.getItem('token');
        console.log('Token na verificação:', typeof token, token);
        const validatoken = await invoke("verifica_token", {email, token});
        console.log(validatoken);
        home();
      } catch(error){
        console.log(error);
      }
    }
  
    async function realizaLogin(){
      try{
        await invoke("verifica_senha", {email, senha});
        setMensagemSenha("Entrando na conta!");    
        const novo_token = await invoke("busca_id", {email}); //Preparando autenticação
        localStorage.setItem('token', novo_token); // Armazenando token
        console.log('Token gerado ao entrar:', novo_token);    
      } catch (error){
        setMensagemSenha("Erro ao entrar na conta. Verifique sua senha."); // Alterar para mensagem de erro personalizada
        return;
      } finally{
        if (localStorage.getItem('token')){ // Se tiver um token definido, faz login direto no menu
          console.log("Token foi definido.")
          home();
        }
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
            
  
            //await buscaID();
            //const idendereco = await cadastraEndereco();
            //cadastraLocadora(idendereco);
  
            //atualizaEmail();
            //atualizaSenha();
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