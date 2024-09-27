import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";
import './redefinicao_senha.css';

function RedefinicaoSenha(){
    console.log(localStorage.getItem('codigoReset'));
    const [codigoUsuario, setcodigoUsuario] = useState("");

    async function verifica(){
        const codigoBanco = localStorage.getItem('codigoReset');
        try{
            const mensagem = await invoke("verifica_codigo_email", {codigoUsuario, codigoBanco});
            console.log(mensagem);
            alteraSenha();
        } catch(error){
            console.log("[Redefinicao_senha.jsx | verifica] : ", error);
        }
    }
    const navigate = useNavigate();

    const alteraSenha = () => {
      navigate('/altera_senha');
    };

    const reset_senha = () =>{
        navigate('/reset_senha');
    };

    return(

    <div className="reset-password">
        <h1>Redefinir Senha</h1>

        <form action="post" onSubmit={async (e) => {
            e.preventDefault();
            await verifica();
            
          }}>
            <input required

          id="reset-password"
          onChange={(e) => setcodigoUsuario(e.currentTarget.value)}
          placeholder="Digite o código" 
        />
            <button type="submit">Verificar Código </button>
        </form>
        <p className="info">Um código foi enviado para o seu e-mail. Verifique sua caixa de entrada e insira o código acima para redefinir sua senha.</p>
        <p >Caso não tenha recebido o codigo. <a href="@">Clique aqui para solicitar o reenvio.</a></p>
        <button onClick={reset_senha}>Voltar</button>
    </div>
    )
}

export default RedefinicaoSenha;