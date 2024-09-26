import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";
import { cadastraEndereco, selecionaUf, selecionaUfDefinido } from "./endereco";
import { cadastraSocioAdm } from "./socioAdm";
import InputMask from 'react-input-mask';
//import "./App.css";


function CriaConta(){

  const [mensagemCriarConta, setMensagemCriarConta] = useState("");

  const [email, setEmail] = useState("");
  const [nomeCompleto, setNomeCompleto] = useState("");
  const [senha1, setSenha1] = useState("");
  const [senha2, setSenha2] = useState("");
  const [cpf, setCpf] = useState("");

  const [cnpj, setCnpj] = useState("");
  const [nomelocadora, setNomeLocadora] = useState("");
  const [numeroconta, setNumeroConta] = useState("");
  const [agenciaconta, setAgenciaConta] = useState("");
  const [nomebanco, setNomeBanco] = useState("");

  const [cep, setCep] = useState("");
  const [cidade, setCidade] = useState("");
  const [numeroendereco, setNumeroEndereco] = useState("");
  const [logradouro, setLogradouro] = useState("");
  const [complemento, setComplemento] = useState("");
  const [uf, setUf] = useState("");

  let idendereco ;
  const [locadoraExiste, setLocadoraExiste] = useState(false);
  const [mensagemEmpresa, setMensagemEmpresa] = useState("");

  const [complementosocio, setComplementoSocio] = useState("");
  const [logradourosocio, setLogradouroSocio] = useState("");
  const [cepsocio, setCepSocio] = useState("");
  const [ufsocio, setUfSocio] = useState("");
  const [cidadesocio, setCidadeSocio] = useState("");
  const [numeroenderecosocio, setNumeroEnderecoSocio] = useState("");

  const [nacionalidade, setNacionalidade] = useState("");
  const [estadocivil, setEstadoCivil] = useState("");
  const [orgaoemissor, setOrgaoEmissor] = useState("");


  async function cadastraLocadora(idsocio){
    try{
      const idendereco = await cadastraEndereco(cep, logradouro, numeroendereco, 
        complemento, cidade, uf);
        const numerocontabanco = numeroconta;
        const numeroagenciabanco = agenciaconta;
        const locadora = await invoke("estrutura_locadora", {idendereco, cnpj, numerocontabanco, numeroagenciabanco, nomelocadora, nomebanco, idsocio});
        await invoke("cadastra_locadora", {locadora});
    } catch(error){
      console.log(error);
      setMensagemCriarConta(error);
      throw(error);
    }
  }
  
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
        setMensagemEmpresa("Sua empresa já está cadastrada! Os dados abaixo foram preenchidos automaticamente.");
      }else{
        setLocadoraExiste(false);
      }
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
      if (!locadoraExiste){
        const idenderecosocio = await cadastraEndereco(cepsocio, logradourosocio, 
          numeroenderecosocio, complementosocio, cidadesocio, ufsocio
        );
        const idsocio = await cadastraSocioAdm(idenderecosocio, nomeCompleto, cpf, 
          orgaoemissor, estadocivil, nacionalidade)
        await cadastraLocadora(idsocio);
      }      
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
          <p>Suas informações</p>
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
            />      
            <InputMask
              mask="999.999.999-99" // Máscara para CPF
              value={cpf}
              onChange={(e) => setCpf(e.target.value)}
              required
              placeholder="CPF"
            >
              {(inputProps) => <input {...inputProps} type="text" />}
            </InputMask> 
            <input
              required
              type="email"
              onChange={(e) => setEmail(e.target.value)}
              placeholder="E-mail" 
            />
          </div>             
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
          <p>{mensagemEmpresa}</p>
          </div>  
          {locadoraExiste ? (
            <div>
            <div><input  readOnly className="user-input" placeholder="Nome da Locadora"  value={nomelocadora || ""} /></div>
            <p>Dados bancários da empresa</p>
            <div><input readOnly className="user-input" placeholder="Nome do banco"  value={nomebanco || ""} /></div>
            <div><input readOnly className="user-input" placeholder="Numero da agencia"  value={agenciaconta || ""} /></div>
            <div><input readOnly className="user-input" placeholder="Numero da conta"  value={numeroconta || ""} /></div>
            <p>Endereço da empresa</p>
            <input readOnly className="user-input" placeholder="CEP"  value={cep || ""} />
            <input readOnly className="user-input" placeholder="Logradouro"  value={logradouro || ""} />
            <input readOnly className="user-input" placeholder="Número"  value={numeroendereco || ""} />
            <input readOnly className="user-input" placeholder="Complemento"  value={complemento || ""} />
            <input readOnly className="user-input" placeholder="Cidade"  value={cidade || ""} />
            {selecionaUfDefinido(setUf, uf, true)}
            <button className="user-input" type="submit">Criar conta</button>

            </div>
) : (
            <div>
              <div><input className="user-input" placeholder="Nome da Locadora"  value={nomelocadora || ""}  onChange={(e) =>setNomeLocadora(e.currentTarget.value)}/></div>
            <p>Dados bancários da empresa</p>
            <div><input className="user-input" placeholder="Nome do banco"  value={nomebanco || ""}  onChange={(e) =>setNomeBanco(e.currentTarget.value)} /></div>
            <div><input className="user-input" placeholder="Numero da agencia"  value={agenciaconta || ""}  onChangeCapture={(e) =>setAgenciaConta(e.currentTarget.value)} /> </div>
            <div><input className="user-input" placeholder="Numero da conta"  value={numeroendereco || ""}  onChange={(e) =>setNumeroConta(e.currentTarget.value)}/></div>
            <p>Endereço da empresa</p>
            <input className="user-input" placeholder="CEP"  value={cep || ""}  onChange={(e) =>setCep(e.currentTarget.value)}/>
            <input className="user-input" placeholder="Logradouro"  value={logradouro || ""}  onChange={(e) =>setLogradouro(e.currentTarget.value)}/>
            <input className="user-input" placeholder="Número"  value={numeroendereco || ""}  onChange={(e) =>setNumeroEndereco(e.currentTarget.value)}/>
            <input className="user-input" placeholder="Complemento"  value={complemento || ""}  onChange={(e) =>setComplemento(e.currentTarget.value)} />
            <input className="user-input" placeholder="Cidade"  value={cidade || ""}  onChange={(e) =>setCidade(e.currentTarget.value)}/>
            {selecionaUf(setUf)}
            <p>Obs.: A locadora precisa de ao menos um sócio cadastrado. Você poderá designar outro sócio após entrar no sistema.</p>
            <p>Cadastre o seu endereço:</p>
            <input className="user-input" placeholder="CEP" onChange={(e) =>setCepSocio(e.currentTarget.value)}/>
            <input className="user-input" placeholder="Logradouro" onChange={(e) =>setLogradouroSocio(e.currentTarget.value)}/>
            <input className="user-input" placeholder="Número"  onChange={(e) =>setNumeroEnderecoSocio(e.currentTarget.value)}/>
            <input className="user-input" placeholder="Complemento" onChange={(e) =>setComplementoSocio(e.currentTarget.value)} />
            <input className="user-input" placeholder="Cidade"  onChange={(e) =>setCidadeSocio(e.currentTarget.value)}/>
            {selecionaUf(setUfSocio)}
            <p></p>
            <p>Continue o cadastro dos seus dados</p>
            <div><input required
          className="user-input"
          onChange={(e) => setOrgaoEmissor(e.currentTarget.value)}
          placeholder="Órgão Emissor do CPF"
          /></div>
          <div>
          <input required
          className="user-input"
          onChange={(e) => setEstadoCivil(e.currentTarget.value)}
          placeholder="Estado civil"
          />
          </div>
          <div>
          <input required
          className="user-input"
          onChange={(e) => setNacionalidade(e.currentTarget.value)}
          placeholder="Nacionalidade"
          />
          </div>
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