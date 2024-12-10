import Layout from "@/layouts/default";
import  "@/components/navbar/navbar.css";
import "@/assets/teste.jpg";
import { useState } from "react";

export default function Login() {
  const [email , setEmail ] = useState ("");
  const [senha , setSenha ] = useState ("");

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
          <input type="email" name="" id="" value={email} onChange={(e) => setEmail(e.target.value)} />
          <input type="password" name="" id="" value={senha} onChange={(e) => setSenha(e.target.value)} />
          <button type="submit" onClick={RealizaLogin}>Entrar</button>
          <button type="submit">Entrar com o Google</button>
          <span>Não possui conta?<a href="">clique aqui</a>para se cadastrar</span>
        </div>
        </div>
        <div>
          <img src="" alt="" />
          
        </div>
      </main>
    </Layout>
  );
}
