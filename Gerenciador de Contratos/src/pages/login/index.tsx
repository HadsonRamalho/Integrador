import "@/components/login/login.css";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { Input } from "@/layouts/Input";
import Layout from "@/layouts/default";
import { Button } from "@/components/ui/button";
//import { loginUser } from "@/services/api/user/user";
import { handleAxiosError } from "@/services/api/error/error";
import { useAuth } from "@/hooks/auth";

export default function AuthPage() {
  const [mode, setMode] = useState("login");
  const [email, setEmail] = useState("");
  const [name, setName] = useState("");
  const [senha, setSenha] = useState("");
  const [document, setDocument] = useState("");
  const [password, setPassword] = useState("");

  const navigate = useNavigate();
  const {signIn} = useAuth();

  const RealizaLogin = async () => {
    try {
      const credentials = { email: email, password: senha };

      await signIn(credentials);
      console.log("Login realizado com sucesso.");
    
      navigate("/");
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } catch (erro: any) {
      console.error("Erro ao realizar login:", erro);
      if (erro.response) {        
        handleAxiosError(erro);
        return;
      } 
      console.error(erro.message || erro);
      alert(erro.message || erro);
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
        
        <div className="container">
          <div className="content firts-content">
          <div className="titulo">
          <h1>MaqExpress</h1>
          <p>
            {mode === "login"
              ? "Faça login para  continuar"
              : "Crie sua conta para acessar"}
          </p>
        </div>
            <div className="login">
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
              <Button className="button" onClick={RealizaLogin}>
                Entrar
              </Button>
              <Button>Entrar com o Google</Button>
              <span>
                Não possui conta?
                <a
                  className="link-login"
                  href="#"
                  onClick={() => setMode("create")}
                >
                  Clique aqui
                </a>
                para se cadastrar
              </span>
            </div>
          </div>

          <div className="login-create">
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
              placeholder="CPF/CNPJ"
              value={document}
              onChange={(e) => setDocument(e.target.value)}
              required
            />
            <Input
              placeholder="Senha"
              value={password}
              type="password"
              onChange={(e) => setPassword(e.target.value)}
            />
            <Button onClick={createAccount}>Criar Conta</Button>
            <span>
              Já possui conta?
              <a
                className="link-login"
                href="#"
                onClick={() => setMode("login")}
              >
                clique aqui!
              </a>
              para entrar
            </span>
          </div>
        </div>
      </main>
    </Layout>
  );
}
