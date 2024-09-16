import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

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
  const [uf, setUfLocadora] = useState("");

  const [nomesocio, setNomeAdmLocadora] = useState("");
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
      console.log(error);
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
      console.log(error);
    }
  }

  async function cadastraEnderecoLocadora(){
    const endereco = await estruturaEnderecoLocadora();
    try{
      const idendereco = await invoke("_salva_endereco", {endereco});
      localStorage.setItem('idendereco', idendereco);
      console.log("Endereço do sócio foi cadastrado");
      return idendereco;
    } catch(error){
      console.log('Erro ao salvar o endereço: ', error);
      setMensagem(error);
    }
  }

  async function estruturaSocioAdm(idendereco){
    try{
      const socio = await invoke("estrutura_socio_adm", {idendereco, nomesocio, cpf, orgaoemissor, estadocivil, nacionalidade});
      return socio;
    } catch(error) {
      setMensagem(error);
      console.log(error);
    }
  }

  async function cadastraSocioAdm(idendereco){
    try{
      const socioadm = await estruturaSocioAdm(idendereco);
      const idsocio = await invoke("cadastra_socio_adm", {socioadm});
      return idsocio;
    } catch(error){
      setMensagem(error);
      console.log(error);
    }
  }

  async function estruturaLocadora(idendereco, idsocio){
    try{
      console.log(idendereco);
      const locadora = await invoke("estrutura_locadora", {idendereco, cnpj, numerocontabanco, numeroagenciabanco, nomebanco, nomelocadora, idsocio});
      return locadora;
    } catch(error) {
      setMensagem(error);
      console.log(error);
    }
  }

  async function cadastraLocadora(idenderecolocadora, idsocio) {
    try{
      const locadora = await estruturaLocadora(idenderecolocadora, idsocio);
      const idlocadora = await invoke("cadastra_locadora", {locadora});
      return idlocadora;
    } catch(error){
      setMensagem(error);
      console.log(error);
    }
  }

  const navigate = useNavigate();

  const home = () => {
    navigate('/home');
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
            const idlocadora = await cadastraLocadora(idenderecolocadora, idsocio);
            setMensagem("Contrato cadastrado!");
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
        <input required
          className="rowReset"
          onChange={(e) => setUfLocadora(e.currentTarget.value)}
          placeholder="UF da Locadora" 
        />
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
        <p>Cadastro do Sócio Administrador</p>
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
        <p>Endereço do Sócio Administrador</p>
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
        <p className="mensagemLogin">{mensagem}</p>
        <button type="submit" >Concluir cadastro</button>
        <br />
        <button onClick={home}>Voltar</button>
        </form>
      </div>
    );
  }

  export default CadastrarContrato;