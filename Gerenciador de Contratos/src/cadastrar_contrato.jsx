import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";
import ChamaContrato from "./pdf_call";
import { useEffect } from "react";


function CadastrarContrato(){
  const [mensagem, setMensagem] = useState("");

  const [nomelocadora, setNomeLocadora] = useState("");
  const [cnpj, setCnpjLocadora] = useState("");

  const [nomebanco, setNomeBanco] = useState("");
  const [numerocontabanco, setNumeroContaBanco] = useState("");
  const [numeroagenciabanco, setNumeroAgenciaBanco] = useState("");

  const [cep, setCepLocadora] = useState("");
  const [cidade, setCidadeLocadora] = useState("");
  const [logradouro, setLogradouroLocadora] = useState("");
  const [numeroendereco, setNumeroLocadora] = useState("");
  const [complemento, setComplementoLocadora] = useState("");
  const [uf , setUfLocadora] = useState("");

  const capturaUfLocadora = (e) => {
    setUfLocadora(e.currentTarget.value); // Atualiza o estado com o valor selecionado
  };

  const [nome, setNomeAdmLocadora] = useState("");
  const [cpf, setCpfAdmLocadora] = useState("")
  const [orgaoemissor, setOrgaoEmissor] = useState("")
  const [estadocivil, setEstadoCivil] = useState("")
  const [nacionalidade, setNacionalidade] = useState("")

  const [cepadm, setCepAdm] = useState("");
  const [cidadeadm, setCidadeAdm] = useState("");
  const [logradouroadm, setLogradouroAdm] = useState("");
  const [numeroenderecoadm, setNumeroAdm] = useState("");
  const [complementoadm, setComplementoAdm] = useState("");
  const [ufadm, setUfAdm] = useState("");

  const [nomemaquina, setNomeMaquina] = useState("");
  const [valoraluguel, setValorAluguel] = useState("");
  const [numserie, setNumSerie] = useState("");

  //contrato
  const [prazolocacao, setPrazoLocacao] = useState("");
  const [dataretirada, setDataRetirada] = useState("");
  const [valormensal, setValorMensal] = useState("");
  const [vencimento, setVencimento] = useState("");
  const [multaatraso, setMultaAtraso] = useState("");
  const [jurosatraso, setJurosAtraso] = useState("");
  const [avisotransferencia, setAvisoTransferencia] = useState("");
  const [prazodevolucao, setPrazoDevolucao] = useState("");
  const [cidadeforo, setCidadeForo] = useState("");
  const [datacontrato, setDataContrato] = useState("");


  //endereco locatario
  const [cepLocatario, setCepLocatario] = useState("");
  const [cidadeLocatario, setCidadeLocatario] = useState("");
  const [logradouroLocatario, setLogradouroLocatario] = useState("");
  const [numeroenderecoLocatario, setNumeroLocatario] = useState("");
  const [complementoLocatario, setComplementoLocatario] = useState("");
  const [ufLocatario, setUfLocatario] = useState("");

  //locatario
  const [cnpjlocatario, setCnpjLocatario] = useState("");
  const [nomelocatario, setNomeLocatario] = useState("");

  //socio locatario
  const [nomesociolocatario, setNomeSocioLocatario] = useState("");
  const [cpfsociolocatario, setCpfSocioLocatario] = useState("");
  const [orgaoemissorsociolocatario, setOrgaoEmissorSocioLocatario] = useState("");
  const [estadocivilsociolocatario, setEstadoCivilSocioLocatario] = useState("");
  const [nacionalidadesociolocatario, setNacionalidadeSocioLocatario] = useState("");

  //endereco socio locatario
  const [cepSocioLocatario, setCepSocioLocatario] = useState("");
  const [cidadeSocioLocatario, setCidadeSocioLocatario] = useState("");
  const [logradouroSocioLocatario, setLogradouroSocioLocatario] = useState("");
  const [numeroenderecoSocioLocatario, setNumeroSocioLocatario] = useState("");
  const [complementoSocioLocatario, setComplementoSocioLocatario] = useState("");
  const [ufSocioLocatario, setUfSocioLocatario] = useState("");

  async function estruturaEnderecoAdm(){
    try{
      const logradouro = logradouroadm;
      const cep = cepadm;
      const complemento = complementoadm;
      const numeroendereco = numeroenderecoadm;
      const cidade = cidadeadm;
      const uf = ufadm;
      const endereco = await invoke("estrutura_endereco", {
        logradouro, 
        cep, 
        complemento, 
        numeroendereco, 
        cidade, 
        uf
      })
      return endereco;
    }
    catch(error){
      setMensagem(error);
      console.log("[Cadastrar_contrato.jsx | estruturaEnderecoAdm] : ", error);
    }
  }

  async function cadastraEnderecoAdm(){
    const endereco = await estruturaEnderecoAdm();
    try{
      const idendereco = await invoke("_salva_endereco", {endereco});
      localStorage.setItem('idendereco', idendereco);
      console.log("Endereço do adm foi cadastrado");
      setMensagem("Endereço do adm foi cadastrado");
      return idendereco;
    } catch(error){
      console.log('Erro ao salvar o endereço: ', error);
      setMensagem("[Cadastrar_contrato.jsx | cadastraEnderecoAdm] : ", error);
    }
  }

  async function estruturaEnderecoAdmLocatario(){
    try{
      const logradouro = logradouroSocioLocatario;
      const cep = cepSocioLocatario;
      const complemento = complementoSocioLocatario;
      const numeroendereco = numeroenderecoSocioLocatario;
      const cidade = cidadeSocioLocatario;
      const uf = ufSocioLocatario;
      const endereco = await invoke("estrutura_endereco", {
        logradouro, 
        cep, 
        complemento, 
        numeroendereco, 
        cidade, 
        uf
      })
      return endereco;
    }
    catch(error){
      setMensagem(error);
      console.log("[Cadastrar_contrato.jsx | estruturaEnderecoAdmLocatario] : ", error);
    }
  }

  async function cadastraEnderecoAdmLocatario(){
    const endereco = await estruturaEnderecoAdmLocatario();
    try{
      const idendereco = await invoke("_salva_endereco", {endereco});
      localStorage.setItem('idendereco', idendereco);
      console.log("Endereço do adm foi cadastrado");
      setMensagem("Endereço do adm foi cadastrado");
      return idendereco;
    } catch(error){
      console.log("[Cadastrar_contrato.jsx | cadastraEnderecoAdmLocatario] : ", error);
      setMensagem(error);
    }
  }

  async function estruturaEnderecoLocatario(){
    try{
      const logradouro = logradouroLocatario;
      const cep = cepLocatario;
      const complemento = complementoLocatario;
      const numeroendereco = numeroenderecoLocatario;
      const cidade = cidadeLocatario;
      const uf = ufLocatario;
      const endereco = await invoke("estrutura_endereco", {
        logradouro, 
        cep, 
        complemento, 
        numeroendereco, 
        cidade, 
        uf
      })
      return endereco;
    }
    catch(error){
      setMensagem(error);
      console.log("[Cadastrar_contrato.jsx | estruturaEnderecoLocatario] : ", error);
    }
  }

  async function cadastraEnderecoLocatario(){
    const endereco = await estruturaEnderecoLocatario();
    try{
      const idendereco = await invoke("_salva_endereco", {endereco});
      localStorage.setItem('idenderecolocatario', idendereco);
      console.log("Endereço do locatario foi cadastrado");
      return idendereco;
    } catch(error){
      console.log("[Cadastrar_contrato.jsx | cadastraEnderecoLocatario] : ", error);
      setMensagem(error);
    }
  }

  async function estruturaEnderecoLocadora(){
    try{
      const endereco = await invoke("estrutura_endereco", {
        logradouro, 
        cep, 
        complemento, 
        numeroendereco, 
        cidade, 
        uf
      })
      return endereco;
    }
    catch(error){
      setMensagem(error);
      console.log("[Cadastrar_contrato.jsx | estruturaEnderecoLocadora] : ", error);
    }
  }

  async function cadastraEnderecoLocadora(){
    const endereco = await estruturaEnderecoLocadora();
    try{
      const idendereco = await invoke("_salva_endereco", {endereco});
      localStorage.setItem('idendereco', idendereco);
      console.log("Endereço da locadora foi cadastrado");
      return idendereco;
    } catch(error){
      console.log("[Cadastrar_contrato.jsx | cadastraEnderecoLocadora] : ", error);
      setMensagem(error);
    }
  }

  async function estruturaSocioAdm(idendereco){
    try{
      const socio = await invoke("estrutura_socio_adm", {idendereco, nome, cpf, orgaoemissor, estadocivil, nacionalidade});
      return socio;
    } catch(error) {
      setMensagem(error);
      console.log("[Cadastrar_contrato.jsx | estruturaSocioAdm] : ", error);
    }
  }

  async function cadastraSocioAdm(idendereco){
    try{
      const socioadm = await estruturaSocioAdm(idendereco);
      const idsocio = await invoke("cadastra_socio_adm", {socioadm});
      return idsocio;
    } catch(error){
      setMensagem(error);
      console.log("[Cadastrar_contrato.jsx | cadastraSocioAdm] : ", error);
    }
  }

  async function estruturaSocioAdmLocatario(idendereco){
    try{
      const nome = nomesociolocatario;
      const cpf = cpfsociolocatario;
      const orgaoemissor = orgaoemissorsociolocatario;
      const estadocivil = estadocivilsociolocatario;
      const nacionalidade = nacionalidadesociolocatario;
      const socio = await invoke("estrutura_socio_adm", {idendereco, nome, cpf, orgaoemissor, estadocivil, nacionalidade});
      return socio;
    } catch(error) {
      setMensagem(error);
      console.log("[Cadastrar_contrato.jsx | estruturaSocioAdmLocatario] : ", error);
    }
  }

  async function cadastraSocioAdmLocatario(idendereco){
    try{
      const socioadm = await estruturaSocioAdmLocatario(idendereco);
      const idsocio = await invoke("cadastra_socio_adm", {socioadm});
      return idsocio;
    } catch(error){
      setMensagem(error);
      console.log("[Cadastrar_contrato.jsx | cadastraSocioAdmLocatario] : ", error);
    }
  }

  async function estruturaLocadora(idendereco, idsocio){
    try{
      console.log(idendereco);
      const locadora = await invoke("estrutura_locadora", {idendereco, cnpj, numerocontabanco, numeroagenciabanco, nomebanco, nomelocadora, idsocio});
      return locadora;
    } catch(error) {
      setMensagem(error);
      console.log("[Cadastrar_contrato.jsx | estruturaLocadora] : ", error);
    }
  }

  async function cadastraLocadora(idenderecolocadora, idsocio) {
    try{
      const locadora = await estruturaLocadora(idenderecolocadora, idsocio);
      const idlocador = await invoke("cadastra_locadora", {locadora});
      return idlocador;
    } catch(error){
      setMensagem(error);
      console.log("[Cadastrar_contrato.jsx | cadastraLocadora] : ", error);
    }
  }

  async function estruturaMaquina(){
    try{      
      const maquina = await invoke("estrutura_maquina", {nomemaquina, valoraluguel, numserie});
      console.log("Valor aluguel: ", maquina.valoraluguel);
      return maquina;
    }
    catch(error){
      console.log("[Cadastrar_contrato.jsx | estruturaMaquina] : ", error);
      setMensagem(error);
    }
  } 

  async function cadastraMaquina(){
    try{
      const maquina = await estruturaMaquina();
      const idmaquina =await invoke("cadastra_maquina", {maquina});
      console.log("idmaquina:", idmaquina);
      setMensagem("Máquina cadastrada!");
      return idmaquina;
    } catch(error){
      console.log("[Cadastrar_contrato.jsx | cadastraMaquina] : ", error);
      setMensagem(error);
    }
  }

  async function estruturaLocatario(idendereco, idsocio){
    try{      
      const cnpj = cnpjlocatario;
      const locatario = await invoke("estrutura_locatario", {idendereco, cnpj, nomelocatario, idsocio});
      return locatario;
    }
    catch(error){
      console.log("[Cadastrar_contrato.jsx | estruturaLocatario] : ", error);
      setMensagem(error);
    }
  } 

  async function cadastraLocatario(idendereco, idsocio){
    try{
      const locatario = await estruturaLocatario(idendereco, idsocio);
      const idlocatario = await invoke("cadastra_locatario", {locatario});
      return idlocatario;
    } catch(error){
      console.log("[Cadastrar_contrato.jsx | cadastraLocatario] : ", error);
      setMensagem(error);
    }
  }

  async function estruturaContrato(idlocatario, idlocador, idmaquina,
    idenderecolocadora, prazolocacao, dataretirada, 
    valormensal, vencimento, multaatraso, 
    jurosatraso, avisotransferencia, prazodevolucao,
    cidadeforo, datacontrato){
      try{
      const enderecoretirada = idenderecolocadora;      
      console.log("IDLOCADOR: ", idlocador);
      const contrato = await invoke("estrutura_contrato", {idlocatario, idlocador, idmaquina,
        enderecoretirada, prazolocacao, dataretirada, 
        valormensal, vencimento, multaatraso, 
        jurosatraso, avisotransferencia, prazodevolucao,
        cidadeforo, datacontrato});
        return contrato;
      } catch(error){
        console.log("[Cadastrar_contrato.jsx | estruturaContrato] : ", error);
      }
  }

  async function cadastraContrato(
      idlocatario, idlocador, idmaquina,
      idenderecolocadora, prazolocacao, dataretirada, 
      valormensal, vencimento, multaatraso, 
      jurosatraso, avisotransferencia, prazodevolucao,
      cidadeforo, datacontrato){
    try{
      const contrato = await estruturaContrato(idlocatario, idlocador, idmaquina,
      idenderecolocadora, prazolocacao, dataretirada, 
      valormensal, vencimento, multaatraso, 
      jurosatraso, avisotransferencia, prazodevolucao,
      cidadeforo, datacontrato);
      console.log(contrato);
      await invoke("cadastra_contrato", {contrato});
      setMensagem("Contrato cadastrado!");
    } catch(error){
      console.log("[Cadastrar_contrato.jsx | cadastraContrato] : ", error);
    }
  } 

  const navigate = useNavigate();

  const home = () => {
    navigate('/home');
  };

  const [idlocador, setIdLocadora] = useState("");
  const [idenderecolocadora, setIdEnderecoLocadora] = useState("");

  const carregaDadosLocadora = async () => {
    const id = localStorage.getItem('token');
    try{
      const cnpjLocador = await invoke("busca_cnpj_usuario", {id});
      const cnpj = cnpjLocador;
      console.log("cnpj no usuario: ", cnpj);
      setCnpjLocadora(cnpj);
      const locadoraExistente = await invoke("locadora_existente", {cnpj});
      setIdLocadora(locadoraExistente.idlocadora);
      setIdEnderecoLocadora(locadoraExistente.idendereco);
      setNomeLocadora(locadoraExistente.nomelocadora);
      setNumeroContaBanco(locadoraExistente.numerocontabanco);
      setNumeroAgenciaBanco(locadoraExistente.numeroagenciabanco);
      setNomeBanco(locadoraExistente.nomebanco);
      const idendereco = locadoraExistente.idendereco;
      try{
        const endereco = await invoke("busca_endereco_id", {idendereco});
        setCepLocadora(endereco.cep);
        setLogradouroLocadora(endereco.logradouro);
        setNumeroLocadora(endereco.numeroendereco);
        setUfLocadora(endereco.uf);
        setCidadeLocadora(endereco.cidade);
        setComplementoLocadora(endereco.complemento);
      } catch(error){
        console.log("Ei, pode ser que o endereço ainda não foi cadastrado!", error);
      }      
    } catch(error){
      console.log("Erro ao buscar o CNPJ cadastrado no Usuário: ", error);
    }
  };

  const carregaDadosLocatario = async () => {
    try{
      const cnpj = cnpjlocatario;
      const idlocatario = await invoke("busca_id_locatario", {cnpj});
      console.log("carregaDadosLocatario 1: ", idlocatario);
      const veclocatario = await invoke("busca_locatario_cnpj", {cnpj});
      const locatario = veclocatario[0];
      setNomeLocatario(locatario.nomelocatario);
      try{
        const idendereco = locatario.idendereco;
        const enderecolocatario = await invoke("busca_endereco_id", {idendereco});
        console.log(enderecolocatario);
        setLogradouroLocatario(enderecolocatario.logradouro);
        setCepLocatario(enderecolocatario.cep);
        setNumeroLocatario(enderecolocatario.numeroendereco);
        setCidadeLocatario(enderecolocatario.cidade);
        setComplementoLocatario(enderecolocatario.complemento);
        setUfLocatario(enderecolocatario.uf);
      } catch(error){
        console.log("Erro ao carregar dados do endereço do locatario: ", error);
      }
    }catch(error){
      console.log("Erro ao carregar dados do locatario: ", error);
    }
  }
  const cpdf = () => {
    navigate('/cpdf', {
      state: {
        //Locadora
        nomelocadora,
        cnpjLocadora: cnpj,

        //Locadora | Banco
        numeroConta: numerocontabanco,
        numeroAgencia: numeroagenciabanco,

        //Locadora | Endereço
        cep,
        cidade,
        logradouro,
        numeroendereco,
        complemento,
        uf,

        //Locadora | Socio
        nomeAdmLocadora: nome,
        cpf,
        orgaoemissor,
        nacionalidade,
        estadocivil,
      },
    });
  };

  useEffect(() => {
    // Função executada após a interface ser carregada
    console.log('Componente foi montado e a interface carregou');
    
    // Pode-se executar qualquer outra lógica aqui, como uma chamada de API
    carregaDadosLocadora();
  }, []); //
  
    return (
      <div id="boxCadastroContrato">
        <div>
        <p className="subtitulo">cadastrar contrato</p>
        </div>      
        <form
          onSubmit={async (e) => {
            e.preventDefault();
            const idenderecoadm = await cadastraEnderecoAdm();            
            const idsocio = await cadastraSocioAdm(idenderecoadm);            

            const idmaquina = await cadastraMaquina();

            const idenderecoadmlocatario = await cadastraEnderecoAdmLocatario();
            const idsociolocatario = await cadastraSocioAdmLocatario(idenderecoadmlocatario);

            const idenderecolocatario = await cadastraEnderecoLocatario();
            const idlocatario = await cadastraLocatario(idenderecolocatario, idsociolocatario);
            await cadastraContrato(
              idlocatario, idlocador, idmaquina, 
              idenderecolocadora, prazolocacao, dataretirada, 
              valormensal, vencimento, multaatraso, jurosatraso,
              avisotransferencia, prazodevolucao, cidadeforo, datacontrato);
              ChamaContrato({ 
                nomelocadora, 
                cnpjLocadora: cnpj, 
                nomeAdmLocadora: nome, 
                numeroConta: numerocontabanco, 
                numeroAgencia: numeroagenciabanco 
              });              
              cpdf();
          }}
        >
        <p>Cadastro da locadora</p>
        <input readOnly={true} required
          className="inputContrato"
          placeholder={cnpj || "CNPJ da Locadora (Ex.: 11.222.333/0001-01)"}
        />
        <br/>
          <input readOnly={true} required
          className="inputContrato"
          placeholder={nomelocadora || "Nome da Locadora (Ex.: Mineração XYZ)"}
        />
        <br></br>
        <p>Cadastro do endereço da locadora</p>
        <input readOnly={true} required
          className="inputContrato"
          placeholder={cep || "CEP da Locadora (Ex.: 40400-400)"}
        />
        <br></br>
        <input readOnly={true} required
          className="inputContrato"
          placeholder={cidade || "Cidade da Locadora (Ex.: Belo Horizonte)"}
        />
        <br></br>
        <input readOnly={true} required
          className="inputContrato"
          placeholder={uf || "Estado da Locadora (Ex.: MG)"}
        />
        <div className="input-box">
                    <label htmlFor="estadoLocadora"></label>
                    <select id="estadoLocadora" readOnly={true}
                        name="estadoLocadora" 
                        value={uf}
                        onChange={capturaUfLocadora}                         
                        aria-label="Selecione o estado da Locadora"                        
                    >
                        <option value="" defaultValue={""}>Selecione o estado da Locadora</option>
                        <option value="AC">AC</option>
                        <option value="AL">AL</option>
                        <option value="AP">AP</option>
                        <option value="AM">AM</option>
                        <option value="BA">BA</option>
                        <option value="CE">CE</option>
                        <option value="ES">ES</option>
                        <option value="GO">GO</option>
                        <option value="MA">MA</option>
                        <option value="MT">MT</option>
                        <option value="MS">MS</option>
                        <option value="MG">MG</option>
                        <option value="PA">PA</option>
                        <option value="PB">PB</option>
                        <option value="PR">PR</option>
                        <option value="PE">PE</option>
                        <option value="PI">PI</option>
                        <option value="RJ">RJ</option>
                        <option value="RN">RN</option>
                        <option value="RS">RS</option>
                        <option value="RO">RO</option>
                        <option value="RR">RR</option>
                        <option value="SC">SC</option>
                        <option value="SP">SP</option>
                        <option value="SE">SE</option>
                        <option value="TO">TO</option>
      </select>
    </div>
        <input readOnly={true} required
          className="inputContrato"
          placeholder={logradouro || "Logradouro da Locadora (Ex.: Avenida Central)" }
        />
        <br></br>
        <input readOnly={true} required
          className="inputContrato"
          placeholder={numeroendereco || "Numero do end. Locadora (Ex.: 101B)"}
        />
        <br></br>
        <input readOnly={true} 
          className="inputContrato"
          placeholder={complemento || "Complemento do end. Locadora (Ex.: Sala 01)"} 
        />
        <p>Cadastro dos Dados Bancários da Locadora</p>
        <input readOnly={true}
          className="inputContrato"
          placeholder={nomebanco || "Nome do banco (Ex.: Banco do Brasil)" }
        />
        <br></br>
        <input readOnly={true}
          className="inputContrato"
          placeholder={numerocontabanco || "Número da conta (Ex.: 3040)" }
        />
        <br></br>
        <input readOnly={true}
          className="inputContrato"
          placeholder={numeroagenciabanco || "Agência da conta (Ex.: 001)" }
        />
        <br></br>
        <p>Cadastro do Sócio Administrador da Locadora</p>
        <input required
          className="inputContrato"
          onChange={(e) => setNomeAdmLocadora(e.currentTarget.value)}
          placeholder="Nome do sócio adm. (Ex.: João Carlos Pinheiro)" 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setCpfAdmLocadora(e.currentTarget.value)}
          placeholder="CPF do sócio adm. (Ex.: 123.456.789-01)" 
        />
        <br></br>
        <input
          className="inputContrato"
          onChange={(e) => setOrgaoEmissor(e.currentTarget.value)}
          placeholder="Órgão Emissor do Doc. (Ex.: PC-MG)"
        />
        <br></br>
        <input
          className="inputContrato"
          onChange={(e) => setEstadoCivil(e.currentTarget.value)}
          placeholder="Estado Civil do Sócio (Ex.: Casado)"
        />
        <br></br>
        <input
          className="inputContrato"
          onChange={(e) => setNacionalidade(e.currentTarget.value)}
          placeholder="Nacionalidade do Sócio (Ex.: Brasileiro)"
        />
        <br></br>
        <p>Endereço do Sócio Administrador da Locadora</p>
        <input required
          className="inputContrato"
          onChange={(e) => setCepAdm(e.currentTarget.value)}
          placeholder="CEP do sócio adm. (Ex.: 40400-400)" 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setCidadeAdm(e.currentTarget.value)}
          placeholder="Cidade do sócio adm. (Ex.: Belo Horizonte)" 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setUfAdm(e.currentTarget.value)}
          placeholder="UF do sócio adm. " 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setLogradouroAdm(e.currentTarget.value)}
          placeholder="Logradouro do sócio adm. (Ex.: Avenida Central)" 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setNumeroAdm(e.currentTarget.value)}
          placeholder="Numero do sócio adm. (Ex.: 101B)" 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setComplementoAdm(e.currentTarget.value)}
          placeholder="Complemento do sócio adm. (Ex.: Sala 01)" 
        />
        <br></br>
        <p>Informações da máquina</p>
        <input required
          className="inputContrato"
          onChange={(e) => setNomeMaquina(e.currentTarget.value)}
          placeholder="Nome da máquina (Ex.: Máquina de Corte)"  
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setNumSerie(e.currentTarget.value)}
          placeholder="Número de série (Ex.: 11444A555B)" 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setValorAluguel(e.currentTarget.value)}
          placeholder="Valor aprox. do aluguel (Ex.: 30000)" 
        />
        <br></br>
        <p>Cadastro da empresa do locatario</p>
        <input
          className="inputContrato"
          onChange={(e) => {
            setCnpjLocatario(e.currentTarget.value); 
            carregaDadosLocatario;}}
          onBlur={carregaDadosLocatario}
          placeholder="CNPJ da Empresa (Ex.: 11.234.567/0001-01)"
        />
        <br></br>
        <input
          className="inputContrato"
          onChange={(e) => setNomeLocatario(e.currentTarget.value)}
          placeholder={nomelocatario||"Nome da Empresa (Ex.: Mineração OPQ)"}
        />
        <p>Cadastro do endereço do sócio administrador do locatario</p>
          <input required
          className="inputContrato"
          onChange={(e) => setCidadeSocioLocatario(e.currentTarget.value)}
          placeholder={cidadeLocatario || "Cidade (Ex.: Belo Horizonte)"} 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setUfSocioLocatario(e.currentTarget.value)}
          placeholder={ufLocatario || "Estado"}
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setCepSocioLocatario(e.currentTarget.value)}
          placeholder={cepLocatario || "CEP (Ex.: 40400-400)" }
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setLogradouroSocioLocatario(e.currentTarget.value)}
          placeholder={logradouroLocatario || "Logradouro (Ex.: Avenida Central)"} 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setNumeroSocioLocatario(e.currentTarget.value)}
          placeholder={numeroenderecoLocatario || "Numero do endereço (Ex.: 101B)"}
        />
        <br></br>
        <input
          className="inputContrato"
          onChange={(e) => setComplementoSocioLocatario(e.currentTarget.value)}
          placeholder={complementoLocatario || "Complemento do endereço (Ex.: Sala 01)"}
        />
        <br></br>
        <p>Cadastro do Sócio Administrador do locatario</p>
        <input
          className="inputContrato"
          onChange={(e) => setNomeSocioLocatario(e.currentTarget.value)}
          placeholder="Nome do Sócio (Ex.: Carlos Figueiredo Rocha)"
        />
        <br></br>
        <input
          className="inputContrato"
          onChange={(e) => setCpfSocioLocatario(e.currentTarget.value)}
          placeholder="CPF do Sócio (Ex.: 321.654.987-10)"
        />
        <br></br>
        <input
          className="inputContrato"
          onChange={(e) => setOrgaoEmissorSocioLocatario(e.currentTarget.value)}
          placeholder="Órgão Emissor do Documento (Ex.: PC-MG)"
        />
        <br></br>
        <input
          className="inputContrato"
          onChange={(e) => setEstadoCivilSocioLocatario(e.currentTarget.value)}
          placeholder="Estado Civil do Sócio (Ex.: Solteiro)"
        />
        <br></br>
        <input
          className="inputContrato"
          onChange={(e) => setNacionalidadeSocioLocatario(e.currentTarget.value)}
          placeholder="Nacionalidade do Sócio (Ex.: Brasileiro)"
        />
        <br></br>
        <p>Cadastro do endereço da empresa do locatario</p>
        <input required
          className="inputContrato"
          onChange={(e) => setCidadeLocatario(e.currentTarget.value)}
          placeholder="Cidade (Ex.: Belo Horizonte)" 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setUfLocatario(e.currentTarget.value)}
          placeholder="Estado"
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setCepLocatario(e.currentTarget.value)}
          placeholder="CEP (Ex.: 40400-400)" 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setLogradouroLocatario(e.currentTarget.value)}
          placeholder="Logradouro (Ex.: Avenida Central)" 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setNumeroLocatario(e.currentTarget.value)}
          placeholder="Numero do endereço (Ex.: 101B)"
        />
        <br></br>
        <input
          className="inputContrato"
          onChange={(e) => setComplementoLocatario(e.currentTarget.value)}
          placeholder="Complemento do endereço (Ex.: Sala 01)"
        />
        <br></br>
        
        <p>Informações do contrato</p>
        <input required
          className="inputContrato"
          onChange={(e) => setPrazoLocacao(e.currentTarget.value)}
          placeholder="Prazo de locação (em meses) (Ex.: 12)" 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setDataRetirada(e.currentTarget.value)}
          placeholder="Data de retirada da máquina (Ex.: 2024-12-01)" 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setValorMensal(e.currentTarget.value)}
          placeholder="Valor mensal do contrato (Ex.: 30000)" 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setVencimento(e.currentTarget.value)}
          placeholder="Vencimento do contrato (Ex.: 2025-12-01)" 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setMultaAtraso(e.currentTarget.value)}
          placeholder="Multa de atraso (Ex.: 10)" 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setJurosAtraso(e.currentTarget.value)}
          placeholder="Juros de atraso (Ex.: 5)" 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setAvisoTransferencia(e.currentTarget.value)}
          placeholder="Aviso de transferência (Ex.: Não aplicável)" 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setPrazoDevolucao(e.currentTarget.value)}
          placeholder="Prazo de devolução (Ex.: 2025-12-31)" 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setCidadeForo(e.currentTarget.value)}
          placeholder="Cidade foro (Ex.: Belo Horizonte)" 
        />
        <br></br>
        <input required
          className="inputContrato"
          onChange={(e) => setDataContrato(e.currentTarget.value)}
          placeholder="Data do contrato (Ex.: 2024-12-01)" 
        />
        <br></br>
        <p className="mensagemLogin">{mensagem}</p>
        <button type="submit" >Concluir cadastro</button>
        <br />
        <button onClick={home}>Voltar</button>
        </form>
      </div>
    );
  }

  export default CadastrarContrato;