import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

function DadosUsuario() {
  const [emailAtual, setEmailAtual] = useState("");
  const [emailFixoInterface, setEmailFixoInterface] = useState("");
  const [emailAntigo, setEmailAntigo] = useState("");

  const [nomeAtual, setNomeAtual] = useState("");
  const [nomeFixoInterface, setNomeFixoInterface] = useState("");
  const [nomeAntigo, setNomeAntigo] = useState("");

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
      setNome(nome);
      setNomeAntigo(nome);
      setNomeAtual(nome);
      setNomeFixoInterface(nome);
      setEmailFixoInterface(email);
    } catch (error) {
      console.error("[Dados_usuario.jsx | dados_usuario] : ", error);
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

  async function atualizaNome(){
    try{
      const id = localStorage.getItem('token');
      const email = await invoke("busca_email_usuario", { id });
      const nome = nomeAtual;
      await invoke("atualiza_nome", {email, nome});
      console.log("Nome antigo: ", nomeAntigo);
      console.log("Nome novo: ", nomeAtual);
      setNomeFixoInterface(nome);
    } catch(error){
      console.log(error);
    }
  }

  async function verifica_usuario_socio_locadora(){
    try{
      const idusuario = localStorage.getItem('token');
      const id = idusuario;
      const cnpj = await invoke("busca_cnpj_usuario", {id});
      await invoke("verifica_usuario_socio_locadora", {idusuario, cnpj});
    } catch(error){
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
          <input placeholder={nome} onChange={(e) => setNomeAtual(e.currentTarget.value)}></input>
          <button onClick={async () => {
            await atualizaNome();
            await dados_usuario();
          }}>Atualizar Nome</button>
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
        <button onClick={verifica_usuario_socio_locadora}> Apagar minha conta </button>
      </div>
    </div>
  );
}

export default DadosUsuario;
