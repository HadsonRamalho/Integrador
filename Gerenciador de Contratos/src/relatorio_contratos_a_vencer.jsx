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

  async function buscaContratos(){
    try{
        const contrato = await invoke("busca_contratos_a_vencer", {idusuario});
        setVetor(contrato);  
        setMensagem(""); 
    } catch(error){
        setVetor([]);
        console.log("[relatorio_contratos.jsx] : ", error);
        setMensagem(error);
    }
  }  

  return (
    <div>
      <div className="boxRelatorioContratos">    
        <button className="botoesHome" type="button" onClick={buscaContratos}>Buscar</button>
        <button className="botoesHome" type="button" onClick={home}>Voltar</button>
        <p>{mensagem}</p>
        <div>
          <ul className="contract-list">
            {vetor.map((contrato, index) => (
              <li key={index} className="contract-item">
                <div className="contract-header">Data de Vencimento: {contrato.vencimento}</div>
                <div className="contract-fields">
                  <div><strong>Prazo Locação:</strong> {contrato.prazolocacao} MESES </div>
                  <div><strong>Data Retirada:</strong> {contrato.dataretirada} </div>
                  <div><strong>Valor Mensal: </strong> R$ {contrato.valormensal} </div>
                  <div><strong>Multa em caso de atraso:</strong> {contrato.multaatraso}% </div>
                  <div><strong>Juros em caso de atraso:</strong> {contrato.jurosatraso}% </div>
                  <div><strong>Aviso de Transferência:</strong> {contrato.avisotransferencia} </div>
                  <div><strong>Prazo de Devolução:</strong> {contrato.prazodevolucao} </div>
                  <div><strong>Cidade Foro:</strong> {contrato.cidadeforo} </div>
                  <div><strong>Data do Contrato:</strong> {contrato.datacontrato} </div>
                </div>
              </li>
            ))}
          </ul>
        </div>
      </div>
    </div>
  );
}

export default RelatorioContratos;
