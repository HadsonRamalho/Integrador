import Layout from "@/layouts/default";
import  "@/components/login/login.css";
import maquina from "@/assets/teste.jpg";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
 import {Input} from "@/components/Input/Input";
 
export default function Login() {
  const [email , setEmail ] = useState ("");
  const [senha , setSenha ] = useState ("");
  const navigate = useNavigate();

  const RealizaLogin = async () => {
    try{
      const res = await fetch("https://g6v9psc0-3003.brs.devtunnels.ms/realiza_login",{
        method: "POST" ,
        headers:{
          'Content-Type' : "application/json"
        }
        , body: JSON.stringify({email , senha})

      });
      if(!res.ok){
        const erro = await res.text();
        console.log(erro);
        throw new Error(erro || "Erro ao realizar o login.");
      }
      console.log("Login realizado.");
      navigate("/");
    }

    catch (erro){
      console.error("Erro no Login: ", erro);

    }
  }
 
  return (
    <Layout>
      <main>
        <div className="titulo">
        <h1>MaqExpress</h1>
        <p>Faça login ou cadastre para continuar</p>
        </div>
        <div className="grid">
        <div className="login">
          <Input 
          type="email" 
          name="" id="" 
           placeholder= "E-mail"
          value={email} 
          onChange={(e) => setEmail(e.target.value)} />

          <Input
           type="password" 
           name="" 
           id="" 
           placeholder="Senha"
            value={senha}
             onChange={(e) => setSenha(e.target.value)} />

         <a className="password" href=""onClick={() => {navigate("/password-recovery")}}>Esqueci a senha</a>
          <button type="submit">Entrar com o Google</button>
          <button type="submit" onClick={RealizaLogin}>Entrar</button>
          <span>Não possui conta?<a href="#" onClick={()=> {navigate("/create-account")}}>clique aqui</a>para se cadastrar</span>
        </div>
        <div>
          <img src={maquina} alt="" />
        </div>
        </div>
      </main>
    </Layout>
    
  );
}
