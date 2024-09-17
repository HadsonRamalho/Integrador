import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

function BuscarContrato(){
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
        console.log("[Buscar_contrato.jsx] : ", error);
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
        <button className="botoesHome" type="button" onClick={buscaContrato}>Buscar</button>
        <button className="botoesHome" type="button" onClick={home}>Voltar</button>
        <p>{mensagem}</p>
        <div>
          <ul className="contract-list">
            {vetor.map((contrato, index) => (
              <li key={index} className="contract-item">
                <div className="contract-header">ID DO CONTRATO: {contrato.idcontrato}</div>
                <div className="contract-fields">
                  <strong>Prazo Locação:</strong> {contrato.prazolocacao} MESES <br />
                  <strong>Data Retirada:</strong> {contrato.dataretirada} <br />
                  <strong>Valor Mensal: </strong> R$ {contrato.valormensal} <br />
                  <strong>Vencimento:</strong> {contrato.vencimento} <br />
                  <strong>Multa Atraso:</strong> {contrato.multaatraso} <br />
                  <strong>Juros Atraso:</strong> {contrato.jurosatraso} <br />
                  <strong>Aviso Transferência:</strong> {contrato.avisotransferencia} <br />
                  <strong>Prazo Devolução:</strong> {contrato.prazodevolucao} <br />
                  <strong>Cidade Foro:</strong> {contrato.cidadeforo} <br />
                  <strong>Data Contrato:</strong> {contrato.datacontrato} <br />
                </div>
              </li>
            ))}
          </ul>
        </div>
      </div>
    </div>
  );
}

export default BuscarContrato;
