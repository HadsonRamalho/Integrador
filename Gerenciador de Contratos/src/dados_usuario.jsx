import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";

function DadosUsuario() {
  const [email, setEmail] = useState("");
  const [nome, setNome] = useState("");
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    async function fetchEmail() {
      try {
        const id = localStorage.getItem('token');
        const email = await invoke("busca_email_usuario", { id });
        const nome = await invoke("busca_nome_usuario", {id});
        setEmail(email);
        setNome(nome);
      } catch (error) {
        console.error("Erro ao buscar dados do usuário:", error);
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
        <h3>Nome: {nome}</h3>
        <h3>E-mail: {email}</h3>
    </div>
  );
}

export default DadosUsuario;
