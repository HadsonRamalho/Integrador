import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

function AtualizaSenha(){
  const [mensagem, setMensagem] = useState("");
  const [senha1, setSenha1] = useState("");
  const [senha2, setSenha2] = useState("");
  const [senhaAtual, setSenhaAtual] = useState("");
  const [email, setEmail] = useState("");

  async function atualizaSenha() {
    const id = localStorage.getItem('token');
    try{
        const email = await invoke("busca_email_usuario", {id});
        setEmail(email);
    }catch(error){
        console.log(error);
        setMensagem(error);
        return;
    }

    try{
        const senha = senhaAtual;
        await invoke("verifica_senha", {email, senha});
    }catch(error){
        console.log(error);
        setMensagem(error);
        return;
    }

    try{
      await invoke("compara_novas_senhas", { senha1, senha2 });
      const novaSenha = senha1;
      await invoke("atualiza_senha", {email, novaSenha});
      setMensagem("Senha alterada!");
    } catch(error){
      setMensagem(error);
      console.log("[Atualiza_senha.jsx] : ", error);
    }
  }

  const navigate = useNavigate();

  const dados_usuario = () => {
    navigate('/dados_usuario');
  };

    return (
      <div id="boxreset">
        <div>
        <p className="subtitulo">atualizar senha</p>
        </div>
        <form
          onSubmit={async (e) => {
            e.preventDefault();
            await atualizaSenha();
          }}
        >
        <input required
          className="rowReset"
          onChange={(e) => setSenhaAtual(e.currentTarget.value)}
          placeholder="Senha atual" 
          type="password"
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setSenha1(e.currentTarget.value)}
          placeholder="Senha nova" 
          type="password"
        />
        <br></br>
        <input required

          className="rowReset"
          onChange={(e) => setSenha2(e.currentTarget.value)}
          placeholder="Confirme a senha nova" 
          type="password"
        />
        <p className="mensagemLogin">{mensagem}</p>
        <button type="submit" >Enviar</button>
        <br />
        <button onClick={dados_usuario}>Voltar</button>
        </form>
      </div>
    );
  }

  export default AtualizaSenha;