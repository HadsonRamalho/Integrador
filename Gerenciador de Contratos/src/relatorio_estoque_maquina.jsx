import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

function RelatorioEstoqueMaquina(){
  const [mensagem, setMensagem] = useState("");
  const [vetor, setVetor] = useState([]);

  const [nomemaquina, setNomeMaquina] = useState("");

  async function buscaMaquina(){
    try{
        const estoquemaquina = await invoke("gera_estoque_por_nome", {nomemaquina});
        setVetor(estoquemaquina);  
        setMensagem(""); 
    } catch(error){
        setVetor([]);
        console.log("[Relatorio_estoque_maquina.jsx] : ", error);
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
        <input
          required          
          className="inputRelatorioEstoqueMaquina"
          onChange={(e) => setNomeMaquina(e.currentTarget.value)}
          placeholder="Buscar por nome da máquina (Nome completo, igual consta no cadastro da máquina)"
        />
        <br/>
        <button className="botoesHome" type="button" onClick={buscaMaquina}>Buscar</button>
        <button className="botoesHome" type="button" onClick={home}>Voltar</button>
        <p>{mensagem}</p>
        <div>
          <ul className="contract-list">
            {vetor.map((estoquemaquina, index) => (
              <li key={index} className="contract-item">
                <div className="contract-header">NOME DA MÁQUINA: {estoquemaquina.nomemaquina}</div>
                <div className="contract-fields">
                  <strong>Quantidade em Estoque:</strong> {estoquemaquina.quantidade} <br />
                </div>
              </li>
            ))}
          </ul>
        </div>
      </div>
    </div>
  );
}

export default RelatorioEstoqueMaquina;
