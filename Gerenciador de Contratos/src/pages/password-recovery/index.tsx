import Layout from "@/layouts/default";
import "@/components/password-recovery/password-recovery.css";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import Input from "@/layouts/Input";
import { Button } from "@/components/ui/button";

import {
  InputOTP,
  InputOTPGroup,
  InputOTPSeparator,
  InputOTPSlot,
} from "@/components/ui/input-otp"

export default function PasswordRecovery() {
  const [email, setEmail] = useState("");
  const [message, setMessage] = useState("");
  const [isUpdating, setIsUpdating] = useState(false);
  const [isCodeInputVisible, setIsCodeInputVisible] = useState(false);
  const [code, setCode] = useState(""); 

  const [isPasswordInputVisible, setIsPasswordInputVisible] = useState(false);
  const [newPassword, setNewPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");

  const navigate = useNavigate();

  const API_URL = "https://g6v9psc0-3003.brs.devtunnels.ms";

  const loadUserId = async () => {
    try {
      const res = await fetch(`${API_URL}/busca_usuario_email/{email}?email=${encodeURIComponent(email)}`, {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
        }
      });

      if (!res.ok) {
        throw new Error(await res.text());
      }

      const userid = await res.json();
      console.log("idusuario: ", userid);
      return userid;

    } catch (error) {
      setTimeout(() => setIsUpdating(false));
      setMessage("Erro ao enviar o código de recuperação.");
      console.error(error);
    }
  }

  const handleSubmit = async () => {
    if (!email || !/\S+@\S+\.\S+/.test(email)) {
      setMessage("Por favor, insira um e-mail válido.");
      return;
    }

    setIsUpdating(true);

    try {
      const res = await fetch(`${API_URL}/envia_codigo_recuperacao`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ email }),
      });

      if (!res.ok) {
        throw new Error(await res.text());
      }

      const idcodigo = await res.json();
      console.log("idcodigo: ", idcodigo[0]);
      
      setTimeout(() => setIsUpdating(false));

      setMessage("Código enviado com sucesso. Verifique seu e-mail!");
      setIsCodeInputVisible(true);
    } catch (error) {
      setTimeout(() => setIsUpdating(false));
      setMessage("Erro ao enviar o código de recuperação.");
      console.error(error);
    }
  };

  const handleCodeSubmit = async () => {
    if (!code) {
      setMessage("Por favor, insira o código recebido.");
      return;
    }

    setIsUpdating(true);

    try {
      const userid = await loadUserId();
      const res = await fetch(`${API_URL}/verifica_codigo_recuperacao`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ idusuario: userid, codigodigitado: code }),
      });

      if (!res.ok) {
        throw new Error(await res.text());
      }

      setMessage("Código verificado com sucesso!");
      setIsCodeInputVisible(false);
      setIsPasswordInputVisible(true);
    } catch (error) {
      setMessage("Erro ao verificar o código.");
      console.error(error);
    } finally {
      setIsUpdating(false);
    }
  };

  const handlePasswordChange = async () => {
    if (!newPassword || newPassword !== confirmPassword) {
      setMessage("As senhas não coincidem ou estão vazias.");
      return;
    }

    setIsUpdating(true);

    try {
      const userid = await loadUserId();
      const res = await fetch(`${API_URL}/redefine_senha_usuario`, {
        method: "PATCH",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify({ idusuario: userid, senha_nova: newPassword}),
      });

      if (!res.ok) {
        throw new Error(await res.text());
      }

      setIsUpdating(false);
      setMessage("Senha alterada com sucesso!");
    } catch (error) {
      setIsUpdating(false);
      setMessage("Erro ao alterar a senha.");
      console.error(error);
    }
  };

  return (
    <>
      <Layout>
        <main>
          <div className="password-recovery">
            <div className="input-box height-[400px]">
              <h2 className="title">Recuperação de senha</h2>
              {(!isCodeInputVisible && !isPasswordInputVisible) && (
                <>                 
                  <Input
                    type="email"
                    value={email}
                    placeholder="E-mail"
                    onChange={(e) => setEmail(e.target.value)}
                    required
                  />
                  <Button onClick={handleSubmit} disabled={isUpdating}>
                    {isUpdating ? "Enviando código..." : "Enviar código"}
                  </Button>
                </>
              )}

              {isCodeInputVisible && (
                <>
                <label>Insira o código que foi enviado para o e-mail:</label>
                  <InputOTP maxLength={4} onChange={setCode}>
                  <InputOTPGroup>
                    <InputOTPSlot index={0} />
                    <InputOTPSlot index={1} />
                  </InputOTPGroup>
                  <InputOTPSeparator />
                  <InputOTPGroup>
                    <InputOTPSlot index={2} />
                    <InputOTPSlot index={3} />
                  </InputOTPGroup>
                </InputOTP>
                  <Button onClick={handleCodeSubmit} disabled={isUpdating}>
                    {isUpdating ? "Verificando código..." : "Verificar código"}
                  </Button>
                </>
              )}

            {isPasswordInputVisible && (
                <>
                  <Input
                    type="password"
                    value={newPassword}
                    placeholder="Nova senha"
                    onChange={(e) => setNewPassword(e.target.value)}
                    required
                  />
                  <Input
                    type="password"
                    value={confirmPassword}
                    placeholder="Confirme a nova senha"
                    onChange={(e) => setConfirmPassword(e.target.value)}
                    required
                  />
                  <Button onClick={handlePasswordChange} disabled={isUpdating}>
                    {isUpdating ? "Alterando senha..." : "Alterar senha"}
                  </Button>
                </>
              )}
              <span>{message}</span>
              <Button onClick={() => {navigate(-1)}}>
                Voltar
              </Button>
            </div>            
          </div>
        </main>
      </Layout>
    </>
  );
}
