import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";


function Home(){
  const [mensagem, setMensagem] = useState("");
  const [nomeMaquina, setnomeMaquina] = useState("");

  async function buscaMaquina(){
    try{
        const nome = await invoke("busca_nome_maquina", {nomeMaquina});
        setMensagem(nome);
    } catch(error){
        console.log(error);
        setMensagem(error);
    }
  }

  const navigate = useNavigate();

  const home = () => {
    navigate('/home');
  };

    return (
      <div>
        
        <div id="boxHome">
       <input required          
            className="rowReset"
            onChange={(e) => setnomeMaquina(e.currentTarget.value)}
            placeholder="Buscar por nome da máquina"
          />
          <p>{mensagem}</p>
       <button className="botoesHome" type="button" onClick={buscaMaquina}>Buscar</button>
       <button className="botoesHome" type="button" onClick={home}>Voltar</button>
       </div>
       
        
        
      </div>
    );
  }

  export default Home;