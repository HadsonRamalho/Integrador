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

  const [filtro, setFiltro] = useState("nome");
  const [valorBusca, setValorBusca] = useState("");

  const navigate = useNavigate();

  const home = () => {
    navigate('/home');
  };

  async function buscaClienteNome(){
    try{
      const nomelocatario = valorBusca;
      console.log(nomelocatario);
      const locatarios = await buscaLocatarioNome(nomelocatario);
      setVetor(locatarios);
      setMensagem("");
    } catch (error){
      setVetor([]);
      setMensagem(error);
      console.log(error);
    }
  }

  async function buscaClienteCnpj(){
    try{
      const cnpj = valorBusca;
      const locatarios = await buscaLocatarioCnpj(cnpj);
      setVetor(locatarios);
      setMensagem("");
    } catch (error){
      setVetor([]);
      setMensagem(error);
      console.log(error);
    }
  }

  async function busca_cliente(){
    if (filtro === "nome") {
      await buscaClienteNome();
      return;
    } 
      await buscaClienteCnpj();
      return;
  }

  return (
    <div>
      <div className="boxBuscaCliente">        
        <div>
        <div>
          Filtro: 
          <select value={filtro} onChange={(e) => setFiltro(e.target.value)}>
            <option value="nome">Nome</option>
            <option value="cnpj">CNPJ</option>
        </select>
        <input
          className="rowReset"
          type="text"
          value={valorBusca}
          onChange={(e) => {setValorBusca(e.currentTarget.value)
            }
          }
          placeholder={`Buscar cliente por ${filtro}`}
        />
        <button onClick={async () => {
          await busca_cliente();
        }
        }>Buscar</button>
      </div>
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
