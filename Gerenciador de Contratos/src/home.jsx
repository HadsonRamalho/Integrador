import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";


function Home(){
  const [mensagem, setMensagem] = useState("");

  const navigate = useNavigate();

  const login = () => {
    navigate('/');
  };

  const dados_usuario = () => {
    navigate('/dados_usuario');
  };

  const buscar_contrato = () => {
    navigate('/buscar_contrato');
  };

  const buscar_cliente = () => {
    navigate('/buscar_cliente');
  };

  const buscar_maquina = () => {
    navigate('/buscar_maquina');
  };

    return (
      <div>
        
        <div id="boxHome">
       <button className="botoesHome" type="button" onClick={() => window.location.href = "formulario.html"}>Criar novo contrato</button>
       <button className="botoesHome" type="button" onClick={dados_usuario}>Meus dados</button>
       <button className="botoesHome" type="button" onClick={buscar_contrato}>Buscar Contrato</button>
       <button className="botoesHome" type="button" onClick={buscar_cliente}>Buscar Cliente</button>
       <button className="botoesHome" type="button" onClick={buscar_maquina}>Buscar Máquina</button>
       <button className="botoesHome" type="button" onClick={login}>Voltar</button>
       </div>
        
        
      </div>
    );
  }

  export default Home;