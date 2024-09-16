import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

function BuscarMaquina(){
  const [mensagem, setMensagem] = useState("");
  const [nomeMaquina, setNomeMaquina] = useState("");
  const [vetor, setVetor] = useState([]);
  const idusuario = localStorage.getItem('token');

  async function buscaMaquina(){
    try{
        const valoraluguel = await invoke("filtra_maquina_nome", {nomeMaquina});
        setVetor(valoraluguel); 
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
      <div className="boxBuscaMaquina">
        <input
          required          
          className="rowReset"
          onChange={(e) => setNomeMaquina(e.currentTarget.value)}
          placeholder="Buscar por nome da máquina"
        />
        <button className="botoesHome" type="button" onClick={buscaMaquina}>Buscar</button>
        <button className="botoesHome" type="button" onClick={home}>Voltar</button>        
        <p>{mensagem}</p>
        <div>
          <ul className="contract-list">
            {vetor.map((maquina, index) => (
              <li key={index} className="contract-item">
                <div className="contract-header">NOME DA MAQUINA: {maquina.nomemaquina}</div>
                <div className="contract-fields">
                  <strong>Número de Série:</strong> {maquina.numserie} <br />
                  <strong>Valor Aluguel: </strong> R$ {maquina.valoraluguel} <br />
                  <strong>Disponibildiade:</strong> {maquina.disponibilidade} <br />
                </div>
              </li>
            ))}
          </ul>
        </div>
      </div>
    </div>
  );
}

export default BuscarMaquina;
