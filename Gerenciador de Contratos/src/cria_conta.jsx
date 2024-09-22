import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";
//import "./App.css";


function CriaConta(){

  const [mensagemCriarConta, setMensagemCriarConta] = useState("");

  const [email, setEmail] = useState("");
  const [nomeCompleto, setNomeCompleto] = useState("");
  const [senha1, setSenha1] = useState("");
  const [senha2, setSenha2] = useState("");
  const [cpf, setCpf] = useState("");

  const [cnpj, setCnpj] = useState("");
  const [nomeLocadora, setNomeLocadora] = useState("");
  const [numeroConta, setNumeroConta] = useState("");
  const [agenciaConta, setAgenciaConta] = useState("");
  const [nomebanco, setNomeBanco] = useState("");

  const [locadoraExiste, setLocadoraExiste] = useState(true);
  const carregaDadosLocadora = async (cnpj) => {
    try{
      const locadoraExistente = await invoke("locadora_existente", {cnpj});
      setNomeLocadora(locadoraExistente.nomelocadora);
      setNumeroConta(locadoraExistente.numerocontabanco);
      setAgenciaConta(locadoraExistente.numeroagenciabanco);
      setNomeBanco(locadoraExistente.nomebanco);
      console.log(locadoraExistente);
      const idendereco = locadoraExistente.idendereco;
      if (locadoraExistente.idlocadora != ""){
        console.log("ID vazio");
        setLocadoraExiste(true);
      }
    } catch(error){
      console.log("Erro ao buscar o CNPJ cadastrado no Usuário: ", error);
    }
  };

  async function criarConta() {
    try{
      await invoke("cria_conta", {nomeCompleto, email, senha1, senha2, cpf, cnpj});
      setMensagemCriarConta("Conta criada");
    }
    catch(error){
      setMensagemCriarConta(error);
      console.log("[Cria_conta.jsx | criarConta] : ",  error);
    }
  }

  const navigate = useNavigate();

  const login = () => {
    navigate('/');
  };

  const cadastroLocadora = () => {
    if (!locadoraExiste){
    return(<div>
      <input required
          className="user-input"
          onChange={(e) => setNomeLocadora(e.currentTarget.value)}
          placeholder="Nome da locadora"
      />
      <button>x</button>
      
    </div>);
    }
    return(<div>
      <input required
          className="user-input"
          onChange={(e) => setNomeLocadora(e.currentTarget.value)}
          placeholder="Locadora existe"
      />
      
    </div>);
  };

    return (
      <div id="boxcriar">
        <p id="subtituloForm"></p>
        <div>
        <p className="subtitulo">Cadastre-se</p>
        </div>
        <form
          className="rowSignUp"
          onSubmit={async (e) => {
            e.preventDefault();
            await criarConta();
          }}
        >
          <div>
          <input required
            className="user-input"
            onChange={(e) => setNomeCompleto(e.currentTarget.value)}
            placeholder="Nome completo"
            />
          </div>
          <div>
          <input required
          className="user-input"
          onChange={(e) => setCpf(e.currentTarget.value)}
          placeholder="Seu CPF"
          />    </div>      
          <div>
          <input required
            className="user-input"
            onChange={(e) => setEmail(e.currentTarget.value)}
            placeholder="E-mail " 
          />
          </div>
          <div>
          <input required
            className="user-input"            
            onChange={(e) => setSenha1(e.currentTarget.value)}
            placeholder="Sua senha"
            type="password"
          />
          </div>
          <div>
          <input required
            className="user-input"
            onChange={(e) => setSenha2(e.currentTarget.value)}
            placeholder="Confirme sua senha"
            type="password"
          />
          </div>
          <p>Informações da empresa</p>
          <div>
          <input required
            className="user-input"
            onChange={(e) => setCnpj(e.currentTarget.value)}
            onBlur={async (e) => { await carregaDadosLocadora(e.currentTarget.value); }}
            placeholder="CNPJ da empresa"
          />
          </div>
          {locadoraExiste ? (
            <div>
            <div><input className="user-input" placeholder="Nome da Locadora"  value={nomeLocadora} /></div>
            <div><input className="user-input" placeholder="Nome do banco"  value={nomeLocadora} /></div>
            <p>Dados bancários da empresa</p>
            <div><input className="user-input" placeholder="Numero da agencia"  value={nomeLocadora} /></div>
            <div><input className="user-input" placeholder="Numero da conta"  value={nomeLocadora} /></div>
            <p>Endereço da empresa</p>
            <input className="user-input" placeholder="CEP"  value={nomeLocadora} />
            <input className="user-input" placeholder="Logradouro"  value={nomeLocadora} />
            <input className="user-input" placeholder="Número"  value={nomeLocadora} />
            <input className="user-input" placeholder="Complemento"  value={nomeLocadora} />
            <input className="user-input" placeholder="Cidade"  value={nomeLocadora} />
            <input className="user-input" placeholder="UF"  value={nomeLocadora} />

            </div>
) : (
            <input required className="user-input" onChange={(e) => setNomeLocadora(e.currentTarget.value)} placeholder="Nome da locadora" />
          )}
          <p className="mensagemLogin"> {mensagemCriarConta} </p>  
        <button className="user-input" type="submit">Criar</button>
        <div>
          <br />
          <button className="botaovoltar" type="button" onClick={login}>Ja tenho conta</button>
        </div>
        </form>
      </div>
    );
  }

  export default CriaConta;