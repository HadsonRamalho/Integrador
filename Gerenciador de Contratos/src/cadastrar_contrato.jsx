import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";
import CPDF from "./pdf_gen";
import ChamaContrato from "./pdf_call";

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

  const cpdf = () => {
    navigate('/cpdf', {
      state: {
        nomelocadora,
        cnpjLocadora: cnpj,
        nomeAdmLocadora: nome,
        numeroConta: numerocontabanco,
        numeroAgencia: numeroagenciabanco,
      },
    });
  };

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
            const idenderecolocadora = await cadastraEnderecoLocadora();
            const idlocador = await cadastraLocadora(idenderecolocadora, idsocio);
            const idmaquina = await cadastraMaquina();

            const idenderecoadmlocatario = await cadastraEnderecoAdmLocatario();
            const idsociolocatario = await cadastraSocioAdmLocatario(idenderecoadmlocatario);

            const idenderecolocatario = await cadastraEnderecoLocatario();
            const idlocatario = await cadastraLocatario(idenderecolocatario, idsociolocatario);
            console.log("idlocador no form: ",idlocador);
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
          <input required
          className="rowReset"
          onChange={(e) => setNomeLocadora(e.currentTarget.value)}
          placeholder="Nome da Locadora" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setCnpjLocadora(e.currentTarget.value)}
          placeholder="CNPJ da Locadora"
        />
        <br></br>
        <p>Cadastro do endereço da locadora</p>
        <input required
          className="rowReset"
          onChange={(e) => setCepLocadora(e.currentTarget.value)}
          placeholder="CEP da Locadora" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setCidadeLocadora(e.currentTarget.value)}
          placeholder="Cidade da Locadora" 
        />
        <br></br>
        <div class="input-box">
                    <label for="estadoLocadora"></label>
                    <select id="estadoLocadora" 
                        name="estadoLocadora" 
                        value={uf}
                        onChange={capturaUfLocadora}
                        required 
                        aria-label="Selecione o estado da Locadora"
                        
                    >
                        <option value="" disabled selected>Selecione o estado da Locadora</option>
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
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setLogradouroLocadora(e.currentTarget.value)}
          placeholder="Logradouro da Locadora" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setNumeroLocadora(e.currentTarget.value)}
          placeholder="Numero do end. Locadora" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setComplementoLocadora(e.currentTarget.value)}
          placeholder="Complemento do end. Locadora" 
        />
        <p>Cadastro dos Dados Bancários da Locadora</p>
        <input required
          className="rowReset"
          onChange={(e) => setNomeBanco(e.currentTarget.value)}
          placeholder="Nome do banco" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setNumeroContaBanco(e.currentTarget.value)}
          placeholder="Número da conta" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setNumeroAgenciaBanco(e.currentTarget.value)}
          placeholder="Agência da conta" 
        />
        <br></br>
        <p>Cadastro do Sócio Administrador da Locadora</p>
        <input required
          className="rowReset"
          onChange={(e) => setNomeAdmLocadora(e.currentTarget.value)}
          placeholder="Nome do sócio adm." 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setCpfAdmLocadora(e.currentTarget.value)}
          placeholder="CPF do sócio adm." 
        />
        <br></br>
        <input
          className="rowReset"
          onChange={(e) => setOrgaoEmissor(e.currentTarget.value)}
          placeholder="Órgão Emissor do Doc."
        />
        <br></br>
        <input
          className="rowReset"
          onChange={(e) => setEstadoCivil(e.currentTarget.value)}
          placeholder="Estado Civil do Sócio"
        />
        <br></br>
        <input
          className="rowReset"
          onChange={(e) => setNacionalidade(e.currentTarget.value)}
          placeholder="Nacionalidade do Sócio"
        />
        <br></br>
        <p>Endereço do Sócio Administrador da Locadora</p>
        <input required
          className="rowReset"
          onChange={(e) => setCepAdm(e.currentTarget.value)}
          placeholder="CEP do sócio adm." 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setCidadeAdm(e.currentTarget.value)}
          placeholder="Cidade do sócio adm." 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setUfAdm(e.currentTarget.value)}
          placeholder="UF do sócio adm." 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setLogradouroAdm(e.currentTarget.value)}
          placeholder="Logradouro do sócio adm." 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setNumeroAdm(e.currentTarget.value)}
          placeholder="Numero do sócio adm." 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setComplementoAdm(e.currentTarget.value)}
          placeholder="Complemento do sócio adm." 
        />
        <br></br>
        <p>Informações da máquina</p>
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
          placeholder="Valor aprox. do aluguel" 
        />
        <br></br>
        <p>Cadastro do endereço do sócio administrador do locatario</p>
          <input required
          className="rowReset"
          onChange={(e) => setCidadeSocioLocatario(e.currentTarget.value)}
          placeholder="Cidade" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setUfSocioLocatario(e.currentTarget.value)}
          placeholder="Estado"
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setCepSocioLocatario(e.currentTarget.value)}
          placeholder="CEP" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setLogradouroSocioLocatario(e.currentTarget.value)}
          placeholder="Logradouro" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setNumeroSocioLocatario(e.currentTarget.value)}
          placeholder="Numero do endereço"
        />
        <br></br>
        <input
          className="rowReset"
          onChange={(e) => setComplementoSocioLocatario(e.currentTarget.value)}
          placeholder="Complemento do endereço"
        />
        <br></br>
        <p>Cadastro do Sócio Administrador do locatario</p>
        <input
          className="rowReset"
          onChange={(e) => setNomeSocioLocatario(e.currentTarget.value)}
          placeholder="Nome do Sócio"
        />
        <br></br>
        <input
          className="rowReset"
          onChange={(e) => setCpfSocioLocatario(e.currentTarget.value)}
          placeholder="CPF do Sócio"
        />
        <br></br>
        <input
          className="rowReset"
          onChange={(e) => setOrgaoEmissorSocioLocatario(e.currentTarget.value)}
          placeholder="Órgão Emissor do Documento"
        />
        <br></br>
        <input
          className="rowReset"
          onChange={(e) => setEstadoCivilSocioLocatario(e.currentTarget.value)}
          placeholder="Estado Civil do Sócio"
        />
        <br></br>
        <input
          className="rowReset"
          onChange={(e) => setNacionalidadeSocioLocatario(e.currentTarget.value)}
          placeholder="Nacionalidade do Sócio"
        />
        <br></br>
        <p>Cadastro do endereço da empresa do locatario</p>
        <input required
          className="rowReset"
          onChange={(e) => setCidadeLocatario(e.currentTarget.value)}
          placeholder="Cidade" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setUfLocatario(e.currentTarget.value)}
          placeholder="Estado"
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setCepLocatario(e.currentTarget.value)}
          placeholder="CEP" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setLogradouroLocatario(e.currentTarget.value)}
          placeholder="Logradouro" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setNumeroLocatario(e.currentTarget.value)}
          placeholder="Numero do endereço"
        />
        <br></br>
        <input
          className="rowReset"
          onChange={(e) => setComplementoLocatario(e.currentTarget.value)}
          placeholder="Complemento do endereço"
        />
        <br></br>
        <p>Cadastro da empresa do locatario</p>
        <input
          className="rowReset"
          onChange={(e) => setCnpjLocatario(e.currentTarget.value)}
          placeholder="CNPJ da Empresa"
        />
        <br></br>
        <input
          className="rowReset"
          onChange={(e) => setNomeLocatario(e.currentTarget.value)}
          placeholder="Nome da Empresa"
        />
        <p>Informações do contrato</p>
        <input required
          className="rowReset"
          onChange={(e) => setPrazoLocacao(e.currentTarget.value)}
          placeholder="Prazo de locação (em meses)" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setDataRetirada(e.currentTarget.value)}
          placeholder="Data de retirada da máquina" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setValorMensal(e.currentTarget.value)}
          placeholder="Valor mensal do contrato" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setVencimento(e.currentTarget.value)}
          placeholder="Vencimento do contrato" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setMultaAtraso(e.currentTarget.value)}
          placeholder="Multa de atraso" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setJurosAtraso(e.currentTarget.value)}
          placeholder="Juros de atraso" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setAvisoTransferencia(e.currentTarget.value)}
          placeholder="Aviso de transferência" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setPrazoDevolucao(e.currentTarget.value)}
          placeholder="Prazo de devolução" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setCidadeForo(e.currentTarget.value)}
          placeholder="Cidade foro" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setDataContrato(e.currentTarget.value)}
          placeholder="Data do contrato" 
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