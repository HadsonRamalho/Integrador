import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

function DadosUsuario() {
  const [emailAtual, setEmailAtual] = useState("");
  const [emailFixoInterface, setEmailFixoInterface] = useState("");
  const [emailAntigo, setEmailAntigo] = useState("");
  const [nome, setNome] = useState("");
  const [loading, setLoading] = useState(true);

  const navigate = useNavigate();

  const home = () => {
    navigate('/home');
  };

  const apagarConta = () => {
    navigate('/apagar_conta');
  };


  const dados_usuario = async () => {
    setLoading(true);
    try {
      const id = localStorage.getItem('token');
      const email = await invoke("busca_email_usuario", { id });
      const nome = await invoke("busca_nome_usuario", { id });
      setEmailAntigo(email);
      setEmailAtual(email);
      setEmailFixoInterface(email);
      setNome(nome);
    } catch (error) {
      console.error("Erro ao buscar dados do usuário:", error);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    dados_usuario();
  }, []);

  if (loading) {
    return <div>Carregando...</div>;
  }

  async function atualizaEmail() {
    try {
      const email = emailAtual;
      await invoke("atualiza_email", { emailAntigo, email });
      console.log("Email antigo:", emailAntigo, "Email novo:", emailAtual);
      setEmailFixoInterface(email);
    } catch (error) {
      console.log(error);
    }
  }

  return (
    <div id="boxDadosUsuario">
      <h1>Dados do Usuário</h1>
      <div>
        <h3>Nome: {nome}</h3>
        <h3>E-mail: {emailFixoInterface}</h3>
      </div>
      <div>
        <div>
          <input placeholder={nome}></input>
          <button>Atualizar Nome</button>
        </div>
        <div>
          <input
            placeholder={emailAtual}
            onChange={(e) => setEmailAtual(e.currentTarget.value)}
          />
          <button onClick={async () => {
            await atualizaEmail();
            await dados_usuario();
          }}>Atualizar E-mail</button>
        </div>
        <input placeholder="**********"></input>
        <button>Atualizar Senha</button>
      </div>
      <button onClick={home}>Voltar</button>
      <div>
        <button> Apagar minha conta </button>
      </div>
    </div>
  );
}

export default DadosUsuario;
