import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import Home from "./home";
import Login from "./login";
import ResetSenha from "./reset_senha";
import CriaConta from "./cria_conta";
import DadosUsuario from './dados_usuario';
import BuscarContrato from './buscar_contrato';
import BuscarCliente from "./buscar_cliente";

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
      </Routes>
    </Router>
  );
}

export default App;
