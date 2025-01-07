import Layout from "@/layouts/default";
import "@/components/create-account/create-account.css";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import {Input} from "@/layouts/Input";

export default function Create() {
  const [name, setName] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [document, setDocument] = useState("");

  const createAccount = async () => {
    try{
      const res = await fetch("https://g6v9psc0-3003.brs.devtunnels.ms/cadastra_usuario",{
        method: "POST" ,
        headers:{
          'Content-Type' : "application/json"
        }, 
        body: JSON.stringify({nome: name, email, senha: password, documento: document})
      });
      if(!res.ok){
        const erro = await res.text();
        console.log("Erro ao tentar criar a conta:", erro);
        throw new Error(erro);
      }
      console.log("Conta criada!");
    }
    catch (erro){
      console.error(erro);
    }
  };

  return (
    <Layout>
      <main>
        <div>
        <h1>crie sua conta</h1>
        <div className="create-account-box">
          <span className="pl-[45%]">Crie sua conta</span>
          <div className="create-account ml-[40px]">
            <Input
              placeholder="Nome"
              value={name}
              onChange={(e) => setName(e.target.value)}
              required
            />
            <Input
              placeholder="E-mail"
              value={email}
              type={'email'}
              onChange={(e) => setEmail(e.target.value)}
              required
            />
            <Input
              placeholder="Senha"
              value={password}
              type={'password'}
              onChange={(e) => setPassword(e.target.value)}
              required
            />
            <Input
              placeholder="Documento"
              value={document}
              onChange={(e) => setDocument(e.target.value)}
              required
            />
            <Button onClick={createAccount}>Criar conta</Button>
          </div>
        </div>
        </div>
      </main>
    </Layout>
  );
}