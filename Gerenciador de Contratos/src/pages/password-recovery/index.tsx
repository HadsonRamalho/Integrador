import Layout from "@/layouts/default";
import "@/pages/password-recovery/password-recovery.css";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

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

      const idcodigo = await res.text();
      console.log("idcodigo: ", idcodigo);

      
      setTimeout(() => setIsUpdating(false), 5000);

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
              <input
                type="email"
                value={email}
                placeholder="E-mail"
                onChange={(e) => setEmail(e.target.value)}
                required
              />
              
              <button 
              onClick={handleSubmit}
              disabled={isUpdating}>
                {isUpdating ? "Enviando código..." : "Enviar código"}
              </button>
              <span>{message}</span>
              <button onClick={() => {navigate(-1)}}>
                Voltar
              </button>
            </div>            
          </div>
        </main>
      </Layout>
    </>
  );
}
