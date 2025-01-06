import Layout from "@/layouts/default";
import "@/components/password-recovery/password-recovery.css";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import Input from "@/layouts/Input";
import { Button } from "@/components/ui/button";

export default function PasswordRecovery() {
  const [email, setEmail] = useState("");
  const [message, setMessage] = useState("");
  const [isUpdating, setIsUpdating] = useState(false);
  const navigate = useNavigate();

  const API_URL = "https://g6v9psc0-3003.brs.devtunnels.ms";

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
      // exibir input pra inserir o código
    } catch (error) {
      setTimeout(() => setIsUpdating(false));
      setMessage("Erro ao enviar o código de recuperação.");
      console.error(error);
    }
  };

  return (
    <>
      <Layout>
        <main>
          <div className="password-recovery">
            <div className="input-box">
              <h2 className="title">Recuperação de senha</h2>
              <Input
                type="email"
                value={email}
                placeholder="E-mail"
                onChange={(e) => setEmail(e.target.value)}
                required
              />
              
              <Button 
              onClick={handleSubmit}
              disabled={isUpdating}>
                {isUpdating ? "Enviando código..." : "Enviar código"}
              </Button>
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
