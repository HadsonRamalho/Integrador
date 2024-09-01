import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function DadosUsuario() {
  const [email, setEmail] = useState("");
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchEmail() {
      try {
        const id = localStorage.getItem('token');
        const email = await invoke("busca_email_usuario", { id });
        setEmail(email); 
      } catch (error) {
        console.error("Erro ao buscar email do usuário:", error);
      } finally {
        setLoading(false);
      }
    }

    fetchEmail();
  }, []);

  if (loading) {
    return <div>Carregando...</div>;
  }

  return (
    <div id="boxDadosUsuario">
      <h3>E-mail: {email}</h3>
    </div>
  );
}

export default DadosUsuario;
