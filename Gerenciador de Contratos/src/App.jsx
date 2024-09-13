import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Home from "./home";
import Login from "./login";
import ResetSenha from "./reset_senha";
import CriaConta from "./cria_conta";
import DadosUsuario from './dados_usuario';
import BuscarContrato from './buscar_contrato';
import BuscarCliente from "./buscar_cliente";
import BuscarMaquina from './buscar_maquina';
import RelatorioContratos from './relatorio_contratos';
import RedefinicaoSenha from './redefinicao_senha';
import AlteraSenha from './altera_senha';
import ApagarConta from './apagar_conta';
import CadastrarLocatario from './cadastrar_locatario';
import CPDF from './pdf_call';

function App() {
  return (
    <Router>
      <Routes>
        <Route path="/" element={<Login />} />
        <Route path="/home" element={<Home />} />
        <Route path="/reset_senha" element={<ResetSenha/>} />
        <Route path="/cria_conta" element={<CriaConta/>} />
        <Route path="/dados_usuario" element={<DadosUsuario/>} />
        <Route path="/buscar_contrato" element={<BuscarContrato/>} />
        <Route path="/buscar_cliente" element={<BuscarCliente/>} />
        <Route path="/buscar_maquina" element={<BuscarMaquina/>} />
        <Route path="/relatorio_contratos" element={<RelatorioContratos/>} />
        <Route path="/redefinicao_senha" element = {<RedefinicaoSenha/>} />
        <Route path="/altera_senha" element = {<AlteraSenha/>} />
        <Route path="/apagar_conta" element = {<ApagarConta/>} />
        <Route path="/cadastrar_locatario" element = {<CadastrarLocatario/>} />
        <Route path="/cpdf" element = {<CPDF/>} />
      </Routes>
    </Router>
  );
}

export default App;
