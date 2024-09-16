import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";
import CadastrarLocatario from "./cadastrar_locatario";

function CadastrarMaquina(){
  const [mensagem, setMensagem] = useState("");

  const [nomemaquina, setNomeMaquina] = useState("");
  const [numserie, setNumSerie] = useState("");
  const [valoraluguel, setValorAluguel] = useState("");

  async function estruturaMaquina(){
    try{      
      const maquina = await invoke("estrutura_maquina", {nomemaquina, numserie, valoraluguel});
      return maquina;
    }
    catch(error){
      console.log(error);
      setMensagem(error);
    }
  } 

  async function cadastraMaquina(){
    try{
      const maquina = await estruturaMaquina();
      await invoke("cadastra_maquina", {maquina});
      setMensagem("Máquina cadastrada!");
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
      <div id="boxCadastroCliente">
        <div>
        <p className="subtitulo">cadastrar maquina</p>
        </div>
        <form
          onSubmit={async (e) => {
            e.preventDefault();
            await cadastraMaquina();
          }}
        >
        <input required
          className="rowReset"
          onChange={(e) => setNomeMaquina(e.currentTarget.value)}
          placeholder="Nome da máquina" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setNumSerie(e.currentTarget.value)}
          placeholder="Número de série" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setValorAluguel(e.currentTarget.value)}
          placeholder="Valor do aluguel" 
        />
        <p className="mensagemLogin">{mensagem}</p>
        <button type="submit" >Concluir cadastro</button>
        <br />
        <button onClick={home}>Voltar</button>
        </form>
      </div>
    );
  }

  export default CadastrarMaquina;