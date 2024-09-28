use mysql_async::Pool;

use crate::{controller::{self, cria_pool, usuario::{atualiza_email, cria_conta, deleta_conta}}, model::usuario::busca_id_usuario};
#[cfg(test)]

async fn cria_usuario_teste(nome: &str, email: &str, senha: &str, cpf: &str, cnpj: &str) -> Result<(), String> {
    controller::usuario::cria_conta(nome, email, senha, senha, cpf, cnpj).await
}

async fn _setup_pool() -> Result<Pool, mysql_async::Error> {
    cria_pool().await
}

async fn _limpa_usuario(idusuario: &str, email: &str) -> Result<(), String> {
    controller::usuario::deleta_conta(idusuario.to_string(), email.to_string()).await
}

async fn _busca_id_usuario(email: &str)  -> Result<String, String>{
    match controller::usuario::busca_id(email).await {
        Ok(idusuario) =>{
            return Ok(idusuario)
        },
        Err(e) => {
            println!("Erro ao buscar o ID do usuário: {}", e);
            return Err(e.to_string());
        }
    };
}

#[tokio::test]
async fn test_cria_deleta_usuario_ok(){
    let email = "usuariotesteX@teste.com";
    let nome_completo = "Usuario Teste";        
    let senha = "senhausuarioteste1.";
    let cpf = "12312312301";
    let cnpj = "11331220000101";
    assert!(cria_usuario_teste(nome_completo, email, senha, cpf, cnpj).await.is_ok(), 
        "Erro ao criar a conta do usuário");
    
    let idusuario = _busca_id_usuario(&email).await;
    assert!(idusuario.is_ok(), "Erro ao buscar o ID do usuário");
    assert!(_limpa_usuario(&idusuario.unwrap(), email).await.is_ok(),
        "Erro ao deletar a conta do usuário");
}

#[tokio::test]
async fn test_busca_nome_usuario_ok() {    
    let email = "usuariotesteY@teste.com";
    let nome_completo = "Usuario Teste";        
    let senha = "senhausuarioteste1.";
    let cpf = "12312312301";
    let cnpj = "11331220000101";
    assert!(cria_usuario_teste(nome_completo, email, senha, cpf, cnpj).await.is_ok(), 
        "Erro ao criar a conta do usuário");

    let idusuario = _busca_id_usuario(&email).await;
    assert!(idusuario.is_ok(), "Erro ao buscar o ID do usuário");
    let idusuario_cpy = idusuario.clone();

    let mock_pool = _setup_pool().await.expect("Erro ao criar a pool");

    let nome_esperado = "Usuario Teste".to_string();
    let resultado_busca = controller::usuario::_busca_nome_usuario(&mock_pool, &idusuario.unwrap()).await;
    assert_eq!(resultado_busca.unwrap(), nome_esperado);

    assert!(_limpa_usuario(&idusuario_cpy.unwrap(), email).await.is_ok(),
        "Erro ao deletar a conta do usuário");
}

#[tokio::test]
async fn test_busca_nome_usuario_id_invalido() {
    let mock_pool = _setup_pool().await.expect("Erro ao criar a pool");

    let resultado_busca = controller::usuario::_busca_nome_usuario(&mock_pool, "$2b$10$").await;
    assert!(resultado_busca.is_err());
}

#[tokio::test]
async fn test_verifica_email_senha(){
    let email = "usuariotesteZ@teste.com";
    let nome_completo = "Usuario Teste";        
    let senha = "senhausuarioteste1.";
    let cpf = "12312312301";
    let cnpj = "11331220000101";
    assert!(cria_conta(nome_completo, email, senha, senha, cpf, cnpj).await.is_ok(),
        "Erro ao criar a conta do usuário");
    assert!(controller::usuario::verifica_senha(email, senha).await.is_ok(),
        "Erro na verificação de senha");
    let idusuario = match _busca_id_usuario(email).await{
        Ok(idusuario) => idusuario,
        Err(e) => {
            println!("Erro ao buscar o ID do usuário: {}", e);
            return;
        }
    };
    assert!(_limpa_usuario(&idusuario, email).await.is_ok(),
        "Erro ao deletar a conta do usuário");
}

#[tokio::test]
async fn test_atualiza_email(){
    let email = "usuarioteste9@teste.com";
    let nome_completo = "Usuario Teste";        
    let senha = "senhausuarioteste1.";
    let cpf = "12312312301";
    let cnpj = "11331220000101";
    assert!(cria_conta(nome_completo, email, senha, senha, cpf, cnpj).await.is_ok());

    assert!(atualiza_email(email.to_string(), "usuariotesteV@teste.com".to_string()).await.is_ok(),
        "Erro ao atualizar o e-mail do usuário");

    let idusuario = match _busca_id_usuario(email).await{
        Ok(idusuario) => idusuario,
        Err(e) => {
            println!("Erro ao buscar o ID do usuário: {}", e);
            return;
        }
    };

    assert!(_limpa_usuario(&idusuario, "usuariotesteV@teste.com").await.is_ok(),
        "Erro ao deletar a conta do usuário");
}
