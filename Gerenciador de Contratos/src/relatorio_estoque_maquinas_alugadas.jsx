import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

function RelatorioEstoqueTotalAlugadas(){
  const [mensagem, setMensagem] = useState("");
  const [vetor, setVetor] = useState([]);

  async function geraRelatorio(){
    try{
        const estoquemaquina = await invoke("gera_estoque_total_alugadas", {});
        setVetor(estoquemaquina);  
        setMensagem(""); 
    } catch(error){
        setVetor([]);
        console.log("[Relatorio_estoque_total.jsx] : ", error);
        setMensagem(error);
    }
  }

  const navigate = useNavigate();

  const home = () => {
    navigate('/home');
  };

  return (
    <div>
      <div id="boxRelatorioEstoqueMaquina">
        <br/>
        <button className="botoesHome" type="button" onClick={geraRelatorio}>Buscar</button>
        <button className="botoesHome" type="button" onClick={home}>Voltar</button>
        <p>{mensagem}</p>
        <div>
          <ul className="contract-list">
            {vetor.map((estoquemaquina, index) => (
              <li key={index} className="contract-item">
                <div className="contract-header">NOME DA MÁQUINA: {estoquemaquina.nomemaquina}</div>
                <div className="contract-fields">
                  <strong>Quantidade alugada:</strong> {estoquemaquina.quantidade} <br />
                </div>
              </li>
            ))}
          </ul>
        </div>
      </div>
    </div>
  );
}

export default RelatorioEstoqueTotalAlugadas;