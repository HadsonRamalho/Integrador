import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

function BuscarMaquina(){
  const [mensagem, setMensagem] = useState("");
  const [nomeMaquina, setNomeMaquina] = useState("");
  const [vetor, setVetor] = useState([]);
  const [numserie, setNumSerie] = useState("");
  const [valorBusca, setValorBusca] = useState("");
  const idusuario = localStorage.getItem('token');
  const [filtro, setFiltro] = useState('nome');


  async function buscaMaquina(){
    try{
      setNumSerie(valorBusca);
      setNomeMaquina(valorBusca);
      console.log(valorBusca);
      if (filtro === "nome") {
        const maquinas = await invoke("busca_maquina_nome", { nomeMaquina: valorBusca });
        setVetor(maquinas); 
        setMensagem("");
        return;
      } 
        const maquina = await invoke("busca_maquina_numserie", { numserie: valorBusca });
        setVetor(maquina); 
        setMensagem("");
        return;
    } catch(error){
        console.log("[Buscar_maquina.jsx] : ", error);
        setVetor([]);
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
         <div>
          Filtro: 
          <select value={filtro} onChange={(e) => setFiltro(e.target.value)}>
            <option value="nome">Nome</option>
            <option value="número de série">Número de série</option>
        </select>
        <input
          className="rowReset"
          type="text"
          value={valorBusca}
          onChange={(e) => {setValorBusca(e.currentTarget.value)
            }
          }
          placeholder={`Buscar máquina por ${filtro}`}
        />
        <button onClick={async () => {
          await buscaMaquina();
        }
        }>Buscar</button>
      </div>
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
                  <strong>Disponibilidade:</strong> {maquina.disponibilidade} <br />
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
