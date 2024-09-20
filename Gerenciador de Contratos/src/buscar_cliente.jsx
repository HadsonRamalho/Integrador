import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";
import { buscaLocatarioCnpj, buscaLocatarioNome } from "./locatario";

function BuscarCliente(){
  const [mensagem, setMensagem] = useState("");
  const [nomelocatario, setNomeLocatario] = useState("");
  const [cnpj, setCnpjLocatario] = useState("");
  const [vetor, setVetor] = useState([]);
  const idusuario = localStorage.getItem('token');
  

  const navigate = useNavigate();

  const home = () => {
    navigate('/home');
  };

  async function buscaClienteNome(){
    try{
      const locatarios = await buscaLocatarioNome(nomelocatario);
      setVetor(locatarios);
      setMensagem("");
    } catch (error){
      setVetor([]);
      setMensagem("Erro: Não há nenhum locatário com esse nome. [ " + error + " ]");
      console.log(error);
    }
  }

  async function buscaClienteCnpj(){
    try{
      const locatarios = await buscaLocatarioCnpj(cnpj);
      setVetor(locatarios);
      setMensagem("");
    } catch (error){
      setVetor([]);
      setMensagem("Erro: Não há nenhum locatário com esse CNPJ. [ " + error + " ]");
      console.log(error);
    }
  }

  return (
    <div>
      <div className="boxBuscaCliente">        
        <div>
          <input
            required          
            className="rowReset"
            onChange={(e) => setNomeLocatario(e.currentTarget.value)}
            placeholder="Buscar pelo Nome do cliente"
          />
          <button className="botoesHome" type="button" onClick={buscaClienteNome}>Buscar</button>
        </div>
        <div>
        <input
            required          
            className="rowReset"
            onChange={(e) => setCnpjLocatario(e.currentTarget.value)}
            placeholder="Buscar pelo CNPJ do cliente"
          />
          <button className="botoesHome" type="button" onClick={buscaClienteCnpj}>Buscar pelo CNPJ</button>
        </div>
        <button className="botoesHome" type="button" onClick={home}>Voltar</button>     
        
        <div>
        {mensagem}   
          <ul className="contract-list">
            {vetor.map((locatario, index) => (
              <li key={index} className="contract-item">
                <div className="contract-header">NOME DO CLIENTE: {locatario.nomelocatario}</div>
                <div className="contract-fields">
                  <strong>CNPJ:</strong> {locatario.cnpj} <br />
                  <strong>Nome do Locatario: </strong> {locatario.nomelocatario} <br />
                </div>
              </li>
            ))}
          </ul>
        </div>
      </div>
    </div>
  );
}

export default BuscarCliente;
