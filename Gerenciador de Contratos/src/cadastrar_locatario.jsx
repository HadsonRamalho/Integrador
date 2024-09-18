import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";

function CadastrarLocatario(){
  const [mensagem, setMensagem] = useState("");

  const [cep, setCep] = useState("");
  const [cidade, setCidade] = useState("");
  const [logradouro, setLogradouro] = useState("");
  const [numeroendereco, setNumero] = useState("");
  const [complemento, setComplemento] = useState("");
  const [uf, setUf] = useState("");

  const [cepLocatario, setCepLocatario] = useState("");
  const [cidadeLocatario, setCidadeLocatario] = useState("");
  const [logradouroLocatario, setLogradouroLocatario] = useState("");
  const [numeroenderecoLocatario, setNumeroLocatario] = useState("");
  const [complementoLocatario, setComplementoLocatario] = useState("");
  const [ufLocatario, setUfLocatario] = useState("");

  const [nome, setNomeSocio] = useState("");
  const [cpf, setCpf] = useState("");
  const [orgaoemissor, setOrgaoEmissor] = useState("");
  const [estadocivil, setEstadoCivil] = useState("");
  const [nacionalidade, setNacionalidade] = useState("");

  const [nomelocatario, setNomeLocatario] = useState("");
  const [cnpj, setCnpj] = useState("");

  async function estruturaEndereco(){
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
      console.log("[Cadastrar_locatario.jsx | estruturaEndereco] : ", error);
    }
  }

  async function estruturaSocioAdm(idendereco){
    try{
      const socio = await invoke("estrutura_socio_adm", {idendereco, nome, cpf, orgaoemissor, estadocivil, nacionalidade});
      return socio;
    } catch(error) {
      setMensagem(error);
      console.log("[Cadastrar_locatario.jsx | estruturaSocioAdm] : ", error);
    }
  }

  async function cadastraSocioAdm(idendereco){
    try{
      const socioadm = await estruturaSocioAdm(idendereco);
      const idsocio = await invoke("cadastra_socio_adm", {socioadm});
      return idsocio;
    } catch(error){
      setMensagem(error);
      console.log("[Cadastrar_locatario.jsx | cadastraSocioAdm] : ", error);
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
      console.log("[Cadastrar_locatario.jsx | estruturaEnderecoLocatario] : ", error);
    }
  }

  async function cadastraEndereco(){
    const endereco = await estruturaEndereco();
    try{
      const idendereco = await invoke("_salva_endereco", {endereco});
      localStorage.setItem('idendereco', idendereco);
      console.log("Endereço do sócio foi cadastrado");
      return idendereco;
    } catch(error){
      console.log("[Cadastrar_locatario.jsx | cadastraEndereco] : ", error);
      setMensagem(error);
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
      console.log("[Cadastrar_locatario.jsx | cadastraEnderecoLocatario] : ", error);
      setMensagem(error);
    }
  }

  async function estruturaLocatario(idendereco, idsocio){
    try{      
      const locatario = await invoke("estrutura_locatario", {idendereco, cnpj, nomelocatario, idsocio});
      return locatario;
    }
    catch(error){
      console.log("[Cadastrar_locatario.jsx | estruturaLocatario] : ", error);
      setMensagem(error);
    }
  } 

  async function cadastraLocatario(idendereco, idsocio){
    try{
      const locatario = await estruturaLocatario(idendereco, idsocio);
      await invoke("cadastra_locatario", {locatario});
      setMensagem("Cliente cadastrado!");
    } catch(error){
      console.log("[Cadastrar_locatario.jsx | cadastraLocatario] : ", error);
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
        <p className="subtitulo">cadastrar cliente</p>
        </div>
        <form
          onSubmit={async (e) => {
            e.preventDefault();
            const idendereco = await cadastraEndereco();
            const idenderecoLocatario = await cadastraEnderecoLocatario();
            const idsocio = await cadastraSocioAdm(idendereco);
            await cadastraLocatario(idenderecoLocatario, idsocio);
          }}
        >
        <p>Cadastro do endereço do sócio administrador</p>
          <input required
          className="rowReset"
          onChange={(e) => setCidade(e.currentTarget.value)}
          placeholder="Cidade" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setUf(e.currentTarget.value)}
          placeholder="Estado"
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setCep(e.currentTarget.value)}
          placeholder="CEP" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setLogradouro(e.currentTarget.value)}
          placeholder="Logradouro" 
        />
        <br></br>
        <input required
          className="rowReset"
          onChange={(e) => setNumero(e.currentTarget.value)}
          placeholder="Numero do endereço"
        />
        <br></br>
        <input
          className="rowReset"
          onChange={(e) => setComplemento(e.currentTarget.value)}
          placeholder="Complemento do endereço"
        />
        <br></br>
        <p>Cadastro do Sócio Administrador</p>
        <input
          className="rowReset"
          onChange={(e) => setNomeSocio(e.currentTarget.value)}
          placeholder="Nome do Sócio"
        />
        <br></br>
        <input
          className="rowReset"
          onChange={(e) => setCpf(e.currentTarget.value)}
          placeholder="CPF do Sócio"
        />
        <br></br>
        <input
          className="rowReset"
          onChange={(e) => setOrgaoEmissor(e.currentTarget.value)}
          placeholder="Órgão Emissor do Documento"
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
        <p>Cadastro do endereço da empresa</p>
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
        <p>Cadastro da empresa</p>
        <input
          className="rowReset"
          onChange={(e) => setCnpj(e.currentTarget.value)}
          placeholder="CNPJ da Empresa"
        />
        <br></br>
        <input
          className="rowReset"
          onChange={(e) => setNomeLocatario(e.currentTarget.value)}
          placeholder="Nome da Empresa"
        />
        <p className="mensagemLogin">{mensagem}</p>
        <button type="submit" >Concluir cadastro</button>
        <br />
        <button onClick={home}>Voltar</button>
        </form>
      </div>
    );
  }

  export default CadastrarLocatario;