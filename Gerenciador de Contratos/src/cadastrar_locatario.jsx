import { useState } from "react";
import { useNavigate } from "react-router-dom";
import {cadastraEndereco, selecionaUf} from "./endereco";
import { cadastraSocioAdm } from "./socioAdm";
import { cadastraLocatario } from "./locatario";
import {selecionaEstadoCivil, selecionaNacionalidade} from "./socioAdm";

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

  const navigate = useNavigate();

  async function cadastraDados(){
    try{
      const idendereco = await cadastraEndereco(cep, logradouro, numeroendereco, complemento, cidade, uf);
      const idenderecoLocatario = await cadastraEndereco(cepLocatario, logradouroLocatario, numeroenderecoLocatario, complementoLocatario, cidadeLocatario, ufLocatario);
      const idsocio = await cadastraSocioAdm(idendereco, nome, cpf, orgaoemissor, estadocivil, nacionalidade, "", cnpj);
      await cadastraLocatario(idenderecoLocatario, idsocio, cnpj, nomelocatario);
      setMensagem("Locatário cadastrado com sucesso!");
    } catch(error){
      setMensagem(error);
      console.log("Erro ao tentar cadastrar dados: ", error);
    }
    
  }

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
            cadastraDados();
          }}
        >
        <p>Cadastro do endereço do sócio administrador</p>
        <input required
          className="rowReset"
          onChange={(e) => setCep(e.currentTarget.value)}
          placeholder="CEP" 
        />
        <br></br>
          <input required
          className="rowReset"
          onChange={(e) => setCidade(e.currentTarget.value)}
          placeholder="Cidade" 
        />
        {selecionaUf(setUf, false, 30, 31)}
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
        {selecionaEstadoCivil(setEstadoCivil, 30, 31)}
        {selecionaNacionalidade(setNacionalidade, 30, 31)}
        <p>Cadastro do endereço da empresa</p>
        <input required
          className="rowReset"
          onChange={(e) => setCidadeLocatario(e.currentTarget.value)}
          placeholder="Cidade" 
        />
        {selecionaUf(setUfLocatario, false, 30, 31)}
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