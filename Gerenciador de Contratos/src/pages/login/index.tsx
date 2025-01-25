import "@/components/login/login.css";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { Input } from "@/layouts/Input";
import Layout from "@/layouts/default";
import maquina from "@/assets/authpage.jpg";
import { Button } from "@/components/ui/button";
//import { loginUser } from "@/services/api/user/user";
import { handleAxiosError } from "@/services/api/error/error";
import { useAuth } from "@/hooks/auth";
import GoogleLoginButton from "@/components/google-login-button";

export default function AuthPage() {
  const [mode, setMode] = useState("login");
  const [email, setEmail] = useState("");
  const [name, setName] = useState("");
  const [senha, setSenha] = useState("");
  const [document, setDocument] = useState("");
  const [password, setPassword] = useState("");

  const navigate = useNavigate();
  const { signIn } = useAuth();

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
    try {
      const res = await fetch(
        "https://g6v9psc0-3003.brs.devtunnels.ms/cadastra_usuario",
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            nome: name,
            email,
            documento: document,
            senha: password,
          }),
        }
      );
      if (!res.ok) {
        const erro = await res.text();
        console.log("Erro ao tentar criar a conta:", erro);
        throw new Error(erro);
      }
      console.log("Conta criada!");
      setMode("login");
    } catch (erro) {
      console.error(erro);
    }
  };

  const redirectGoogle = async () => {
    console.log('Redirecionando para o Google...');
    const redirectUri = 'https://g6v9psc0-5173.brs.devtunnels.ms/auth/google/callback';
    const clientId = '853000099698-mja71sb0chsva2m9eu3prpktl31psg5q.apps.googleusercontent.com';
  
    const url = `https://accounts.google.com/o/oauth2/v2/auth?scope=email&response_type=code&redirect_uri=${encodeURIComponent(redirectUri)}&client_id=${clientId}`;
    console.log('URL gerada:', url);
  
    window.location.href = url;
  };
  
  const loginGoogle = async () => {
    
  }

  return (
    <Layout>
      <main>
        <div className="container">
          <div className="content ">
          <div className="titulo">
                <h1 className="titulo-maqexpresse" style={{color: 'white'}}> MaqExpress</h1>
                <p style={{color: 'white'}}>
                  {mode === "login"
                    ? "Faça login para  continuar"
                    : "Crie sua conta para acessar"}
                </p>
              </div>
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
                    style={{color: 'white'}}
                    className="password"
                    href="#"
                    onClick={() => navigate("/password-recovery")}
                  >
                    Esqueci a senha
                  </a>
                  <Button style={{color: 'white'}} className="button" onClick={RealizaLogin}>
                    Entrar
                  </Button>
                  <GoogleLoginButton onClick={redirectGoogle}></GoogleLoginButton>
                  <span style={{color: 'white'}} className="link">
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
                    className="input-login"
                    onChange={(e) => setName(e.target.value)}
                    required
                  />
                  <Input
                    placeholder="E-mail"
                    value={email}
                    type="email"
                    className="input-login"
                    onChange={(e) => setEmail(e.target.value)}
                    required
                  />
                  <Input
                    placeholder="CPF/CNPJ"
                    value={document}
                    className="input-login"
                    onChange={(e) => setDocument(e.target.value)}
                    required
                  />
                  <Input
                    placeholder="Senha"
                    value={password}
                    type="password"
                    className="input-login"
                    onChange={(e) => setPassword(e.target.value)}
                  />
                  <Button onClick={createAccount}>Criar Conta</Button>
                  <span className="link">
                    Já possui conta?{" "}
                    <a
                      className="link-login"
                      href="#"
                      onClick={() => setMode("login")}
                    >
                      clique aqui!
                    </a>{" "}
                    para entrar
                  </span>
                </>
              )}
            </div>
          </div>
          <div>
            <img src={maquina} alt="" />
          </div>
        </div>
      </main>
    </Layout>
  );
}
