import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";


function Home(){
  const [mensagemReset, setMensagemReset] = useState("");

  const navigate = useNavigate();

  const login = () => {
    navigate('/');
  };

  const dados_usuario = () => {
    navigate('/dados_usuario');
  };

    return (
      <div>
        
        <div id="boxHome">
       <button className="botoesHome" type="button" onClick={() => window.location.href = "formulario.html"}>Criar novo contrato</button>
       <button className="botoesHome" type="button" onClick={dados_usuario}>Meus dados</button>
       <button className="botoesHome" type="button" onClick={login}>Voltar</button>
       </div>
        
        
      </div>
    );
  }

  export default Home;