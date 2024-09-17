import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

function BuscarCliente(){
  const [mensagem, setMensagem] = useState("");
  const [nomelocatario, setNomeLocatario] = useState("");
  const [vetor, setVetor] = useState([]);
  const idusuario = localStorage.getItem('token');
  async function buscaCliente(){
    try{
        const locatario = await invoke("busca_locatario_nome", {nomelocatario});
        setVetor(locatario);
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
      <div className="boxBuscaCliente">
        <input
          required          
          className="rowReset"
          onChange={(e) => setNomeLocatario(e.currentTarget.value)}
          placeholder="Buscar pelo Nome do cliente"
        />
        <button className="botoesHome" type="button" onClick={buscaCliente}>Buscar</button>
        <button className="botoesHome" type="button" onClick={home}>Voltar</button>     
        
        <div>
        {mensagem}   
          <ul className="contract-list">
            {vetor.map((locatario, index) => (
              <li key={index} className="contract-item">
                <div className="contract-header">ID DO CLIENTE: {locatario.idlocatario}</div>
                <div className="contract-fields">
                  <strong>ID do Endereço:</strong> {locatario.idendereco} <br />
                  <strong>CNPJ:</strong> {locatario.cnpj} <br />
                  <strong>Nome do Locatario: </strong> {locatario.nomelocatario} <br />
                  <strong>ID do Socio:</strong> {locatario.idsocio} <br />
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
