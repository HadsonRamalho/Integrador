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
  const [endereco, setEndereco] = useState();
  const [uf, setUf] = useState("");

  const [nomeSocio, setNomeSocio] = useState("");
  const [cpf, setCpf] = useState("");
  const [orgaoEmissor, setOrgaoEmissor] = useState("");
  const [estadoCivil, setEstadoCivil] = useState("");
  const [nacionalidade, setNacionalidade] = useState("");

  const [nomeLocatario, setNomeLocatario] = useState("");
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
    setEndereco(endereco);
    }
    catch(error){
      setMensagem(error);
      console.log(error);
    }
    finally{
      return endereco;
    }
  }

  async function cadastraEndereco(){
    const endereco = await estruturaEndereco();
    try{
      const idendereco = await invoke("_salva_endereco", {endereco});
      localStorage.setItem('idendereco', idendereco);
      return idendereco;
    } catch(error){
      console.log('Erro ao salvar o endereço: ', error);
      setMensagem(error);
    }
  }

  async function estruturaLocatario(idendereco){
    const cnpj = "52123";
    const nomelocatario = "SeuLocatario";
    try{
      const locatario = await invoke("estrutura_locatario", {idendereco, cnpj, nomelocatario, idsocio});
    }
    catch(error){
      console.log(error);
    }
    finally{
      return locatario;
    }
  } 

  async function cadastraLocatario(idendereco){
    try{
      await estruturaLocatario(idendereco);

    } catch(error){
      console.log(error);
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
            await cadastraEndereco();
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