import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from 'react-router-dom';


async function DadosUsuario(){
    const id = localStorage.getItem('token');
    const email = await invoke("busca_email_usuario", {id});
    return (
      <div id="boxDadosUsuario">
       <h3>E-mail: </h3>
        
      </div>
    );
  }

  export default DadosUsuario;