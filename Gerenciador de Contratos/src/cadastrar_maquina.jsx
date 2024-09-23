import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { cadastraMaquina } from "./maquina";

function CadastrarMaquina(){
  const [mensagem, setMensagem] = useState("");

  const [nomemaquina, setNomeMaquina] = useState("");
  const [numserie, setNumSerie] = useState("");
  const [valoraluguel, setValorAluguel] = useState("");

  const navigate = useNavigate();

  const home = () => {
    navigate('/home');
  };

  async function cadastraDados(){
    try{
      await cadastraMaquina(nomemaquina, valoraluguel, numserie);
      setMensagem("Máquina cadastrada com sucesso!");
    } catch(error){
      setMensagem(error);
      console.log(error);
    }
  }

  const formataValor = (e) => {
    let valor = e.currentTarget.value.replace(/\D/g, "");
    valor = (Number(valor) / 100).toLocaleString("pt-BR", {
      style: "currency",
      currency: "BRL",
    });
    setValorAluguel(valor);
  };

    return (
      <div id="boxCadastroMaquina">
        <div>
        <p className="subtitulo">cadastrar maquina</p>
        </div>
        <form
          onSubmit={async (e) => {
            e.preventDefault();
            cadastraDados();
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
          placeholder="R$ 0,00" 
          type="text"
          value={valoraluguel}
          onChange={formataValor}
          
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