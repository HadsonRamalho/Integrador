import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

function AlteraSenha(){
  const [mensagemReset, setMensagemAlteracao] = useState("");
  const [senha1, setSenha1] = useState("");
  const [senha2, setSenha2] = useState("");

  async function alteraSenha() {
    try{
      const codigo = await invoke("compara_novas_senhas", { senha1, senha2 });
      localStorage.setItem('codigoReset', codigo);
      const email = localStorage.getItem('emailReset');
      const novaSenha = senha1;
      await invoke("atualiza_senha", {email, novaSenha});
      setMensagemAlteracao("Senha alterada!");
    } catch(error){
      setMensagemAlteracao(error);
      console.log(error);
    }
  }

  const navigate = useNavigate();

  const login = () => {
    navigate('/');
  };

    return (
      <div id="boxreset">
        <div>
        <p className="subtitulo">alterar senha</p>
        </div>
        <form
          onSubmit={async (e) => {
            e.preventDefault();
            await alteraSenha();
          }}
        >
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
        <p className="mensagemLogin">{mensagemReset}</p>
        <button type="submit" >Enviar</button>
        <br />
        <button onClick={login}>Voltar para o login</button>
        </form>
      </div>
    );
  }

  export default AlteraSenha;