import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

function RelatorioContratos(){
  const [mensagem, setMensagem] = useState("");
  const [vetor, setVetor] = useState([]);
  const idusuario = localStorage.getItem('token');

  const navigate = useNavigate();

  const home = () => {
    navigate('/home');
  };

  return (
    <div>
      <div className="boxRelatorioContratos">    
        <button className="botoesHome" type="button" onClick={home}>Voltar</button>        
        <p>{mensagem}</p>        
      </div>
    </div>
  );
}

export default RelatorioContratos;
