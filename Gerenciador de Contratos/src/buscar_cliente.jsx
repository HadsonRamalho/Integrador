import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

function BuscarCliente(){
  const [mensagem, setMensagem] = useState("");
  const [nomelocatario, setNomeLocatario] = useState("");
  const [cnpj, setCnpjLocatario] = useState("");
  const [vetor, setVetor] = useState([]);
  const idusuario = localStorage.getItem('token');
  async function buscaCliente(){
    try{
        const locatario = await invoke("busca_locatario_nome", {nomelocatario});
        setVetor(locatario);
        setMensagem("");
    } catch(error){
        console.log(error);
        setMensagem("[Buscar_cliente.jsx | BuscarCliente] : ", error);
    }
  }

  async function buscaClienteCnpj(){
    try{
        const locatario = await invoke("busca_locatario_cnpj", {cnpj});
        setVetor(locatario);
        setMensagem("");
    } catch(error){
        console.log(error);
        setMensagem("[Buscar_cliente.jsx | buscaClienteCnpj] : ", error);
    }
  }

  const navigate = useNavigate();

  const home = () => {
    navigate('/home');
  };

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
          <button className="botoesHome" type="button" onClick={buscaCliente}>Buscar</button>
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
