import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

function Home(){
  const [mensagem, setMensagem] = useState("");
  const [nomeMaquina, setNomeMaquina] = useState("");
  const [vetor, setVetor] = useState([]);
  const idusuario = localStorage.getItem('token');
  async function buscaContrato(){
    try{
        const contrato = await invoke("filtra_contrato_nome_maquina", {nomeMaquina, idusuario});
        setVetor(contrato);  
        setMensagem(""); 
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
      <div id="boxBuscaContrato">
        <input
          required          
          className="rowReset"
          onChange={(e) => setNomeMaquina(e.currentTarget.value)}
          placeholder="Buscar por nome da máquina"
        />
        <p>{mensagem}</p>
        <button className="botoesHome" type="button" onClick={buscaContrato}>Buscar</button>
        <button className="botoesHome" type="button" onClick={home}>Voltar</button>
        <div>
        <ul>
          {vetor.map((contrato, index) => (
            <li key={index}>
              <strong>idcontrato:</strong> {contrato.idcontrato} <br />
              <strong>prazolocacao:</strong> {contrato.prazolocacao} <br />
              <strong>dataretirada:</strong> {contrato.dataretirada} <br />
              <strong>valormensal:</strong> {contrato.valormensal} <br />
              <strong>vencimento:</strong> {contrato.vencimento} <br />
              <strong>multaatraso:</strong> {contrato.multaatraso} <br />
              <strong>jurosatraso:</strong> {contrato.jurosatraso} <br />
              <strong>avisotransferencia:</strong> {contrato.avisotransferencia} <br />
              <strong>prazodevolucao:</strong> {contrato.prazodevolucao} <br />
              <strong>cidadeforo:</strong> {contrato.cidadeforo} <br />
              <strong>datacontrato:</strong> {contrato.datacontrato} <br />
              <strong>dataretirada:</strong> {contrato.dataretirada} <br />
            </li>
          ))}
        </ul>
        </div>
      </div>
    </div>
  );
}

export default Home;
