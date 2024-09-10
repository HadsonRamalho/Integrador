import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

function BuscarCliente(){
  const [mensagem, setMensagem] = useState("");
  const [cnpjlocatario, setCnpj] = useState("");
  const [vetor, setVetor] = useState([]);
  const idusuario = localStorage.getItem('token');
  async function buscaCliente(){
    try{
        const nomelocatario = await invoke("busca_nome_locatario", {cnpjlocatario});
        setMensagem(nomelocatario); 
        setVetor(nomelocatario);
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
          onChange={(e) => setCnpj(e.currentTarget.value)}
          placeholder="Buscar por CNPJ do cliente"
        />
        <button className="botoesHome" type="button" onClick={buscaCliente}>Buscar</button>
        <button className="botoesHome" type="button" onClick={home}>Voltar</button>        
        <div>
          <ul>
            {vetor.map((locatario, index) => (
              <li key={index}>
                <div >ID DO CLIENTE: {locatario.idlocatario}</div>
                <div >
                  <strong>ID do Endereço:</strong> {locatario.idendereco} <br />
                  <strong>CNPJ:</strong> {locatario.cnpj} <br />
                  <strong>Nome do Locatario: </strong> R$ {locatario.nomelocatario} <br />
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
