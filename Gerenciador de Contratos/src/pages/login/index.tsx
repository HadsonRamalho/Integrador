import "@/components/login/login.css";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { Input } from "@/layouts/Input";
import maquina from "@/assets/authpage.jpg";
import { Button } from "@/components/ui/button";
//import { loginUser } from "@/services/api/user/user";
import { handleAxiosError } from "@/services/api/error/error";
import { useAuth } from "@/hooks/auth";
import GoogleLoginButton from "@/components/google-login-button";
import { createUser } from "@/services/api/user/user";
import { UserInput } from "@/interfaces/user";

export default function AuthPage() {
  const [mode, setMode] = useState("login");
  const [email, setEmail] = useState("");
  const [emailS, setEmailS] = useState("");
  const [name, setName] = useState("");
  const [senha, setSenha] = useState("");
  const [document, setDocument] = useState("");
  const [password, setPassword] = useState("");

  const [cadastrando, setCadastrando] = useState(false);
  const [entrando, setEntrando] = useState(false);

  const navigate = useNavigate();
  const { signIn } = useAuth();

  const RealizaLogin = async () => {
    try {
      if (!email || !senha) {
        alert("Preencha todos os campos.");
        return;
      }
      setEntrando(true);
      const credentials = { email: email, password: senha };

      await signIn(credentials);
      console.log("Login realizado com sucesso.");
      setEntrando(false);

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
      setEntrando(false);
    }
  };

  const createAccount = async () => {
    if (!name || !emailS || !password || !document) {
      alert("Preencha todos os campos");
      return;
    }
    try {
      setCadastrando(true);
      const data: UserInput = {
        nome: name,
        documento: document,
        senha: password,
        email: emailS,
      };
      await createUser(data);
      alert("Sua conta foi criada! Você já pode fazer login.");
      setCadastrando(false);
      setMode("login");
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
    } catch (erro: any) {
      console.error(erro);
      setCadastrando(false);
      alert(`Houve um erro ao tentar criar sua conta. ${erro.message}`);
    }
  };

  const redirectGoogle = async () => {
    console.log("Redirecionando para o Google...");
    const redirectUri = "http://localhost:5173/auth/google/callback";
    const clientId =
      "853000099698-mja71sb0chsva2m9eu3prpktl31psg5q.apps.googleusercontent.com";

    const url = `https://accounts.google.com/o/oauth2/v2/auth?scope=email&response_type=code&redirect_uri=${encodeURIComponent(redirectUri)}&client_id=${clientId}`;
    console.log("URL gerada:", url);

    window.location.href = url;
  };

  return (
    <main>
      <div className="container ml-0 md:ml-[6%]">
        <div className="content ">
          <div className="titulo">
            <h1> MaqExpress</h1>
            <p>
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
                  className="password"
                  onClick={() => navigate("/password-recovery")}
                >
                  Esqueci a senha
                </a>
                <Button
                  disabled={entrando}
                  className="button"
                  onClick={RealizaLogin}
                >
                  {entrando ? "Entrando..." : "Entrar"}
                </Button>
                <GoogleLoginButton onClick={redirectGoogle}></GoogleLoginButton>
                <span className="link">
                  Não possui conta?{" "}
                  <a
                    className="link-login hover:cursor-pointer"
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
                  value={emailS}
                  type="email"
                  className="input-login"
                  onChange={(e) => setEmailS(e.target.value)}
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
                <Button onClick={createAccount} disabled={cadastrando}>
                  {cadastrando ? "Cadastrando..." : "Criar Conta"}
                </Button>
                <span className="link">
                  Já possui conta?{" "}
                  <a
                    className="link-login  hover:cursor-pointer"
                    onClick={() => setMode("login")}
                  >
                    Clique aqui!
                  </a>{" "}
                  para entrar
                </span>
              </>
            )}
          </div>
        </div>
        <div className="imagem-login">
          <img src={maquina} alt="" />
        </div>
      </div>
    </main>
  );
}
