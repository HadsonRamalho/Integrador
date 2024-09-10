import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";
import './redefinicao_senha.css';

function RedefinicaoSenha(){
   return(

    <div className="reset-password">
        <h1>Redefinir Senha</h1>

        <form action="post">
            <input id="reset-password" type="text" name="reset-password" placeholder="Insira o código recebido" />
            <button type="submit">Verificar Código </button>
        </form>
        <p className="info">Um código foi enviado para o seu e-mail. Verifique sua caixa de entrada e insira o código acima para redefinir sua senha.</p>
        <p >Caso não tenha recebido o codigo. <a href="@">Clique aqui para solicitar o reenvio.</a></p>
    </div>
    )
}

export default RedefinicaoSenha;