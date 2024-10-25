use axum::{http::StatusCode, Json};

use crate::controller::{self, usuario::DeletaContaInput, EmailInput};

#[cfg(test)]

#[tokio::test]
async fn test_e2e_ok(){
    use crate::test::{maquina::{_limpa_maquina, cria_maquina_teste}, usuario::{_busca_id_usuario, _limpa_usuario, cria_usuario_teste}};

    let email = "usuarioteste1@teste.com";
    let nome_completo = "Usuario Teste";        
    let senha = "senhausuarioteste1.";
    let cpf = "12312312301";
    let cnpj = "11331220000101";
    assert!(cria_usuario_teste(nome_completo, email, senha, cpf, cnpj).await.is_ok());
    let idusuario = _busca_id_usuario(&email).await;
    assert!(idusuario.is_ok(), "Erro ao buscar o ID do usuário");

    assert!(_limpa_usuario(&idusuario.unwrap(), email).await.is_ok(),
        "Erro ao deletar a conta do usuário");

    let nomemaquina = "Maquina de Corte";
    let numserie = "MQC123-TEST-E2E0";
    let valoraluguel = "R$ 4000,00";
    let maquina = cria_maquina_teste(nomemaquina, numserie, valoraluguel).await.unwrap();
    assert!(_limpa_maquina(numserie).await.is_ok())
    
}