import  "@/components/login/login.css";
import maquina from "@/assets/teste.jpg";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
 import {Input} from "@/layouts/Input";
import Layout from "@/layouts/default";
import { Button } from "@/components/ui/button";
//import { loginUser } from "@/services/api/user/user";
import { handleAxiosError } from "@/services/api/error/error";
import { useAuth } from "@/hooks/auth";
 
export default function AuthPage() {
  const [mode, setMode] = useState("login"); 
  const [email , setEmail ] = useState ("");
  const [name , setName] = useState("");
  const [senha , setSenha ] = useState ("");
  const [document, setDocument] = useState("");
  const [password, setPassword] = useState("");
  const navigate = useNavigate();
  const {signIn} = useAuth();
  const RealizaLogin = async () => {
    
    try{
      const credentials = { email: email, password: senha};
      signIn(credentials);
      console.log("Login realizado.");
      navigate("/");
    }
    catch (erro){
      handleAxiosError(erro);
    }
  };

    const createAccount = async () => {
      try{
        const res = await fetch ("https://g6v9psc0-3003.brs.devtunnels.ms/cadastra_usuario",{
        method: "POST",
        headers:{
          'Content-Type' : "application/json"
        },
        body: JSON.stringify({nome: name , email , documento: document, senha: password})
      });
      if(!res.ok){
        const erro = await res.text();
        console.log("Erro ao tentar criar a conta:", erro);
        throw new Error(erro);
      }
      console.log("Conta criada!");
      setMode("login");
      
      }
      catch (erro){
        console.error(erro);
      }
    };
  
  
 
   return (
    <Layout>
      <main>
        <div>
        <div className="titulo">
        {/* <img className="imagem" src={maq} alt="" /> */}
          <h1>MaqExpress</h1>
          <p>{mode === "login" ? "Faça login para continuar" : "Crie sua conta para acessar"}</p>
        </div>

        <div className="grid">
          <div className="login">
            {mode === "login" ? (
              <>
                <Input
                  type="email"
                  placeholder="E-mail"
                  value={email}
                  className="input-login"
                  onChange={(e) => setEmail(e.target.value)}
                />
                <Input
                  type="password"
                  placeholder="Senha"
                  value={senha}
                  className="input-login"
                  onChange={(e) => setSenha(e.target.value)}
                />
                <a
                  className="password"
                  href="#"
                  onClick={() => navigate("/password-recovery")}
                >
                  Esqueci a senha
                </a>
                <Button type="button" onClick={RealizaLogin}>
                  Entrar
                </Button>
                <span>
                  Não possui conta?{" "}
                  <a
                    className="link-login"
                    href="#"
                    onClick={() => setMode("create")}
                  >
                    Clique aqui
                  </a>{" "}
                  para se cadastrar
                </span>
              </>
            ) : (
              <>
                <Input
                  placeholder="Nome"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  required
                />
                <Input
                  placeholder="E-mail"
                  value={email}
                  type="email"
                  onChange={(e) => setEmail(e.target.value)}
                  required
                />
                <Input
                  placeholder="Senha"
                  value={password}
                  type="password"
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
                <span>
                  Já possui conta?{" "}
                  <a
                    className="link-login"
                    href="#"
                    onClick={() => setMode("login")}
                  >
                    Clique aqui
                  </a>{" "}
                  para entrar
                </span>
              </>
            )}
          </div>
          <div>
            <img src={maquina} alt="Máquina" />
          </div>
        </div>
        </div>
      </main>
    </Layout>
  );
}