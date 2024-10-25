use axum::{http::StatusCode, Json};

use crate::controller::{self, usuario::DeletaContaInput, EmailInput};

#[cfg(test)]

#[tokio::test]
async fn test_e2e_ok(){
    use crate::test::{endereco::{_limpa_endereco, cria_endereco_teste}, locadora::{_limpa_locadora, cria_locadora_teste}, maquina::{_limpa_maquina, cria_maquina_teste}, socioadm::{_limpa_socio, cria_socio_teste}, usuario::{_busca_id_usuario, _limpa_usuario, cria_usuario_teste}};

    let email = "usuariotestee2e1@teste.com";
    let nome_completo = "Usuario Teste";        
    let senha = "senhausuarioteste1.";
    let cpf = "12312312301";
    let cnpj = "11331220000101";
    assert!(cria_usuario_teste(nome_completo, email, senha, cpf, cnpj).await.is_ok());
    let idusuario = _busca_id_usuario(&email).await;
    assert!(idusuario.is_ok(), "Erro ao buscar o ID do usuário");

    let logradouro = "Rua K";
    let cep = "39600-000";
    let complemento = "Z";
    let numeroendereco = "515";
    let cidade = "Araçuaí";
    let uf: &str = "MG";
    let idenderecosocio = cria_endereco_teste(logradouro, cep, complemento, numeroendereco, cidade, uf).await.unwrap();

    let orgaoemissor = "PCMG";
    let estadocivil = "Solteiro";
    let nacionalidade = "Brasileiro";
    let idsocio = cria_socio_teste(&idenderecosocio ,nome_completo,cpf,orgaoemissor,estadocivil,nacionalidade,cnpj).await;

    let logradouro = "Rua X";
    let cep = "39600-000";
    let complemento = "Y";
    let numeroendereco = "55";
    let cidade = "Araçuaí";
    let uf = "MG";
    let idenderecolocadora = cria_endereco_teste(logradouro, cep, complemento, numeroendereco, cidade, uf).await.unwrap();

    let nomebanco = "Banco do Brasil";
    let numerocontabanco = "12345";
    let numeroagenciabanco = "1234";
    let nomelocadora = "Locadora Z";

    let locadora = cria_locadora_teste(
        cnpj, &idenderecolocadora, numerocontabanco, numeroagenciabanco, nomebanco, nomelocadora, &idsocio
    ).await.unwrap();
    let idlocadora = locadora.idlocadora.to_string();
    

    let nomemaquina = "Maquina de Corte";
    let numserie = "MQC123-TEST-E2E0";
    let valoraluguel = "R$ 4000,00";
    assert!(cria_maquina_teste(nomemaquina, numserie, valoraluguel).await.is_ok());
    

    assert!(_limpa_maquina(numserie).await.is_ok());
    assert!(_limpa_usuario(&idusuario.unwrap(), email).await.is_ok(),
        "Erro ao deletar a conta do usuário");
    assert!(_limpa_locadora(idlocadora).await.is_ok());
    assert!(_limpa_endereco(idenderecolocadora).await.is_ok());
    assert!(_limpa_socio(idsocio).await.is_ok());
    assert!(_limpa_endereco(idenderecosocio).await.is_ok());
}