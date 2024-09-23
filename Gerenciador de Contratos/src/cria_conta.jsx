import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";
import { cadastraEndereco } from "./endereco";
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

  const [cep, setCep] = useState("");
  const [cidade, setCidade] = useState("");
  const [numeroendereco, setNumeroEndereco] = useState("");
  const [logradouro, setLogradouro] = useState("");
  const [complemento, setComplemento] = useState("");
  const [uf, setUf] = useState("");

  let idendereco ;
  const [locadoraExiste, setLocadoraExiste] = useState(true);

  const cadastraLocadora = async () => {
    const idendereco = 
      await cadastraEndereco(cep, logradouro, numeroendereco, complemento, cidade, uf);
    const numerocontabanco = numeroConta;
    const numeroagenciabanco = agenciaConta;
    await invoke("estrutura_locadora", {idendereco, cnpj, numerocontabanco, numeroagenciabanco, nomeLocadora, nomebanco});
  };
  
  const carregaDadosLocadora = async (cnpj) => {
    setLocadoraExiste(false);
    try{
      const locadoraExistente = await invoke("locadora_existente", {cnpj});
      setNomeLocadora(locadoraExistente.nomelocadora);
      setNumeroConta(locadoraExistente.numerocontabanco);
      setAgenciaConta(locadoraExistente.numeroagenciabanco);
      setNomeBanco(locadoraExistente.nomebanco);
      const id = locadoraExistente.idendereco;
      idendereco = id;

      const idlocadora = locadoraExistente.idlocadora;
      if (idlocadora != ""){
        setLocadoraExiste(true);
      }else{
        setLocadoraExiste(false);
      }
      console.log(locadoraExiste);
    } catch(error){
      console.log("Erro ao buscar o CNPJ cadastrado no Usuário: ", error);
    }

    try{
      const endereco = await invoke("busca_endereco_id", {idendereco});
      setUf(endereco.uf);
      setComplemento(endereco.complemento);
      setLogradouro(endereco.logradouro);
      setCidade(endereco.cidade);
      setCep(endereco.cep);
      setNumeroEndereco(endereco.numeroendereco);
     } catch(error){
      console.log(error);
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
            <div><input  readOnly className="user-input" placeholder="Nome da LAocadora"  value={nomeLocadora} /></div>
            <p>Dados bancários da empresa</p>
            <div><input readOnly className="user-input" placeholder="Nome do banco"  value={nomebanco} /></div>
            <div><input readOnly className="user-input" placeholder="Numero da agencia"  value={agenciaConta} /></div>
            <div><input readOnly className="user-input" placeholder="Numero da conta"  value={numeroConta} /></div>
            <p>Endereço da empresa</p>
            <input readOnly className="user-input" placeholder="CEP"  value={cep} />
            <input readOnly className="user-input" placeholder="Logradouro"  value={logradouro} />
            <input readOnly className="user-input" placeholder="Número"  value={numeroendereco} />
            <input readOnly className="user-input" placeholder="Complemento"  value={complemento} />
            <input readOnly className="user-input" placeholder="Cidade"  value={cidade} />
            <input readOnly className="user-input" placeholder="UF"  value={uf} />
            <button className="user-input" type="submit">Criar</button>

            </div>
) : (
            <div>
              <div><input className="user-input" placeholder="Nome da LocadorAAa"  onChange={(e) =>setNomeLocadora(e.currentTarget.value)}/></div>
            <p>Dados bancários da empresa</p>
            <div><input className="user-input" placeholder="Nome do banco"  onChange={(e) =>setNomeBanco(e.currentTarget.value)} /></div>
            <div><input className="user-input" placeholder="Numero da agencia" onChangeCapture={(e) =>setAgenciaConta(e.currentTarget.value)} /> </div>
            <div><input className="user-input" placeholder="Numero da conta"  onChange={(e) =>setNumeroConta(e.currentTarget.value)}/></div>
            <p>Endereço da empresa</p>
            <input className="user-input" placeholder="CEP" onChange={(e) =>setCep(e.currentTarget.value)}/>
            <input className="user-input" placeholder="Logradouro" onChange={(e) =>setLogradouro(e.currentTarget.value)}/>
            <input className="user-input" placeholder="Número"  onChange={(e) =>setNumeroEndereco(e.currentTarget.value)}/>
            <input className="user-input" placeholder="Complemento" onChange={(e) =>setComplemento(e.currentTarget.value)} />
            <input className="user-input" placeholder="Cidade"  onChange={(e) =>setCidade(e.currentTarget.value)}/>
            <input className="user-input" placeholder="UF (Ex.: MG, SP, RJ)" onChange={(e) =>setUf(e.currentTarget.value)} />
            <button className="user-input" type="submit">Criar conta e cadastrar Locadora</button>
              
              </div>
          )}
          <p className="mensagemLogin"> {mensagemCriarConta} </p>  
        <div>
          <br />
          <button className="botaovoltar" type="button" onClick={login}>Ja tenho conta</button>
        </div>
        </form>
      </div>
    );
  }

  export default CriaConta;