import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

function AlteraSenha(){
  const [mensagemReset, setMensagemAlteracao] = useState("");
  const [senha1, setSenha1] = useState("");
  const [senha2, setSenha2] = useState("");

  async function alteraSenha() {

    try{
      const response = await fetch('http://localhost:3000/compara_novas_senhas', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({senha1, senha2}),
    });

    if (!response.ok) {
      const errorMessage = await response.text();
      throw new Error(`Erro: ${errorMessage}`);
    }
    
    const codigo = await response.text();
    console.log(codigo);

    localStorage.setItem('codigoReset', codigo);
    const email = localStorage.getItem('emailReset');
    const novaSenha = senha1;

    const response2 = await fetch('http://localhost:3000/atualiza_senha', {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({email, senha: novaSenha}),
    });

    if (!response2.ok) {
      const errorMessage = await response.text();
      throw new Error(`Erro: ${errorMessage}`);
    }

    console.log("Senha alterada!");
    setMensagemAlteracao("Senha alterada!");
  } catch (error) {
    setMensagemAlteracao(error);
    console.error('Erro ao verificar o token:', error);      
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