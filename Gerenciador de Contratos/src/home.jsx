import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { useNavigate } from "react-router-dom";
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';




function Home(){
  const [mensagem, setMensagem] = useState("");

  const navigate = useNavigate();

  const login = () => {
    navigate('/');
  };

  const dados_usuario = () => {
    navigate('/dados_usuario');
  };

  const buscar_contrato = () => {
    navigate('/buscar_contrato');
  };

  const buscar_cliente = () => {
    navigate('/buscar_cliente');
  };

  const buscar_maquina = () => {
    navigate('/buscar_maquina');
  };

  const relatorio_contratos = () => {
    navigate('/relatorio_contratos_a_vencer');
  };

  const cadastrar_locatario = () => {
    navigate('/cadastrar_locatario');
  };
    
  const cpdf = () => {
    navigate('/cpdf');
  };

  const cadastrar_contrato = () => {
    navigate('/cadastrar_contrato');
  };

  const cadastrar_maquina = () => {
    navigate('/cadastrar_maquina');
  };

  const relatorio_estoque_maquina = () => {
    navigate('/relatorio_estoque_maquina');
  };

  const relatorio_estoque_total = () => {
    navigate('/relatorio_estoque_total');
  };

  const relatorio_estoque_maquinas_alugadas = () => {
    navigate('/relatorio_estoque_maquinas_alugadas');
  };
  // <button className="botoesHome" type="button" onClick={() => window.location.href = "formulario.html"}>[DEV] Formulário anterior</button>
    return (
      <div>        
        <div id="boxHome">
          <p>Contrato</p>
          <button className="botoesHome" type="button" onClick={cadastrar_contrato}>Cadastrar contrato</button>
          <button className="botoesHome" type="button" onClick={buscar_contrato}>Buscar Contrato</button>
          <button className="botoesHome" type="button" onClick={cpdf}>[DEV] Contrato padrão</button>

          <p>Máquina</p>
          <button className="botoesHome" type="button" onClick={cadastrar_maquina}>Cadastrar maquina</button>
          <button className="botoesHome" type="button" onClick={buscar_maquina}>Buscar Máquina</button>
          <p>Cliente (Locatário)</p>
          <button className="botoesHome" type="button" onClick={cadastrar_locatario}>Cadastrar cliente (Locatario)</button>
          <button className="botoesHome" type="button" onClick={buscar_cliente}>Buscar Cliente (Locatario)</button>
          <p>Relatórios</p>
          <button className="botoesHome" type="button" onClick={relatorio_contratos}>Todos os contratos a vencer</button>
          <button className="botoesHome" type="button" onClick={relatorio_estoque_total}>Todas as máquinas disponíveis</button>
          <button className="botoesHome" type="button" onClick={relatorio_estoque_maquina}>Máquinas em estoque</button>
          <button className="botoesHome" type="button" onClick={relatorio_estoque_maquinas_alugadas}>Todas as máquinas alugadas</button>
          <p>Usuário</p>
          <button className="botoesHome" type="button" onClick={dados_usuario}>Meus dados</button>
          <p>Outros</p>
          <div>
          <button type="button" onClick={async () => {
            const permissao = await requestPermission();
            if (isPermissionGranted) {
              sendNotification({ title: 'Titulo :)', body: 'Texto da notificação :D' });
            }
          }}>
            Testar notificação
          </button>
          </div>
          <br/>
          <button className="botoesHome" type="button" onClick={login}>Sair da conta</button>
        </div> 
      </div>
    );
  }

  export default Home;