use axum::Json;
use mysql_async::Pool;
use axum::http::StatusCode;
use crate::{controller::{self, cria_pool, usuario::{self, atualiza_email, atualiza_senha, cria_conta, deleta_conta, verifica_senha, AtualizaEmailInput, AtualizaNomeInput, DeletaContaInput, UsuarioInput, VerificaSenhaInput, VerificaTokenInput}, EmailInput}, model::{self, usuario::busca_id_usuario}};
#[cfg(test)]

pub async fn cria_usuario_teste(nome: &str, email: &str, senha: &str, cpf: &str, cnpj: &str) -> Result<StatusCode, (StatusCode, Json<String>)>{
    
    use crate::controller::usuario::UsuarioInput;
    let usuario = UsuarioInput{
        nome: nome.to_string(),
        email: email.to_string(),
        senha1: senha.to_string(),
        senha2: senha.to_string(),
        cpf: cpf.to_string(),
        cnpj: cnpj.to_string(),
    };
    let usuario = Json(usuario);
    let res = controller::usuario::cria_conta(usuario).await;
    match res{
        Ok(r) => {
            return Ok(r)
        },
        Err(e) => {
            println!("{}", e.1.0);
            return Err(e)
        }
    }
}

fn estrutura_usuario(nome: &str, email: &str, senha: &str, cpf: &str, cnpj: &str) -> Json<UsuarioInput> {
    use axum::Json;

    use crate::controller::usuario::{UsuarioInput};
    let usuario = UsuarioInput{
        nome: nome.to_string(),
        email: email.to_string(),
        senha1: senha.to_string(),
        senha2: senha.to_string(),
        cpf: cpf.to_string(),
        cnpj: cnpj.to_string(),
    };
    let usuario = Json(usuario);
    usuario
}

async fn _setup_pool() -> Result<Pool, mysql_async::Error> {
    cria_pool().await
}

pub async fn _limpa_usuario(idusuario: &str, email: &str) -> Result<(), (StatusCode, String)> {
    let input = DeletaContaInput{idusuario: idusuario.to_string(), email: email.to_string()};
    controller::usuario::deleta_conta(Json(input)).await
}

pub async fn _busca_id_usuario(email: &str)  -> Result<String, String>{
    match controller::usuario::busca_id(Json(EmailInput{email: email.to_string()})).await {
        Ok(idusuario) =>{
            return Ok(idusuario.1.0)
        },
        Err(e) => {
            return Err(e.1.to_string());
        }
    };
}

#[tokio::test]
pub async fn test_cria_deleta_usuario_ok(){
    let email = "usuarioteste1@teste.com";
    let nome_completo = "Usuario Teste";        
    let senha = "senhausuarioteste1.";
    let cpf = "111.111.111-01";
    let cnpj = "11.131.113/0001-01";
    assert!(cria_usuario_teste(nome_completo, email, senha, cpf, cnpj).await.is_ok());
    let idusuario = _busca_id_usuario(&email).await;
    assert!(idusuario.is_ok(), "Erro ao buscar o ID do usuário");
    assert!(_limpa_usuario(&idusuario.unwrap(), email).await.is_ok(),
        "Erro ao deletar a conta do usuário");
}

#[tokio::test]
async fn test_busca_nome_usuario_ok() {    
    let email = "usuarioteste2@teste.com";
    let nome_completo = "Usuario Teste";        
    let senha = "senhausuarioteste1.";
    let cpf = "12312312301";
    let cnpj = "11331220000101";
    assert!(cria_usuario_teste(nome_completo, email, senha, cpf, cnpj).await.is_ok());

    let idusuario = _busca_id_usuario(&email).await;
    assert!(idusuario.is_ok(), "Erro ao buscar o ID do usuário");
    let idusuario_cpy = idusuario.clone();

    let mock_pool = _setup_pool().await.expect("Erro ao criar a pool");

    let nome_esperado = "Usuario Teste".to_string();
    let resultado_busca = model::usuario::busca_nome_usuario(&mock_pool, &idusuario.unwrap()).await;
    assert_eq!(resultado_busca.unwrap(), nome_esperado);

    assert!(_limpa_usuario(&idusuario_cpy.unwrap(), email).await.is_ok(),
        "Erro ao deletar a conta do usuário");
}

#[tokio::test]
async fn test_busca_nome_usuario_id_invalido_ok() {
    let mock_pool = _setup_pool().await.expect("Erro ao criar a pool");

    let resultado_busca = model::usuario::busca_nome_usuario(&mock_pool, "$2b$10$").await;
    assert!(resultado_busca.is_err());
}

#[tokio::test]
async fn test_verifica_email_senha_ok(){
    let email = "usuarioteste3@teste.com";
    let nome_completo = "Usuario Teste";        
    let senha = "senhausuarioteste1.";
    let cpf = "12312312301";
    let cnpj = "11331220000101";
    let usuario = estrutura_usuario(&nome_completo, &email, &senha, &cpf, &cnpj);
    assert!(cria_conta(Json(usuario.0)).await.is_ok());
    let verifica_senha_input = VerificaSenhaInput{
        email: email.to_string(),
        senha: senha.to_string()
    };
    let resultado_verificacao = controller::usuario::verifica_senha(Json(verifica_senha_input)).await;
    assert!(resultado_verificacao.is_ok());
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
async fn test_atualiza_email_ok(){
    let email = "usuarioteste4@teste.com";
    let nome_completo = "Usuario Teste";        
    let senha = "senhausuarioteste1.";
    let cpf = "12312312301";
    let cnpj = "11331220000101";
    let usuario = estrutura_usuario(&nome_completo, &email, &senha, &cpf, &cnpj);
    assert!(cria_conta(usuario).await.is_ok());

    let pool = match _setup_pool().await{
        Ok(pool) => {pool},
        Err(e) => {
            println!("Erro ao criar a pool: {}", e);
            return;
        }
    };
    let novo_email = "usuariotesteA@teste.com";
    
    assert!(model::busca_email(novo_email).await.is_err(),
        "Erro ao buscar e-mail existente");
    let emailinput = AtualizaEmailInput{email_antigo: email.to_string(), email_novo: novo_email.to_string()};
    assert!(atualiza_email(Json(emailinput)).await.is_ok(),
        "Erro ao atualizar o e-mail do usuário");

    let idusuario = match _busca_id_usuario(novo_email).await{
        Ok(idusuario) => idusuario,
        Err(e) => {
            println!("Erro ao buscar o ID do usuário: {}", e);
            return;
        }
    };

    assert!(_limpa_usuario(&idusuario, novo_email).await.is_ok(),
        "Erro ao deletar a conta do usuário");
}

#[tokio::test]
async fn test_atualiza_senha_ok(){
    let email = "usuarioteste5@teste.com";
    let nome_completo = "Usuario Teste";        
    let senha = "senhausuarioteste1.";
    let cpf = "12312312301";
    let cnpj = "11331220000101";
    let usuario = estrutura_usuario(&nome_completo, &email, &senha, &cpf, &cnpj);
    assert!(cria_conta(usuario).await.is_ok());
    let nova_senha = "novasenhausuarioteste1.";
    let atualiza_senha_input = VerificaSenhaInput{email: email.to_string(), senha: nova_senha.to_string()};
    assert!(atualiza_senha(Json(atualiza_senha_input)).await.is_ok(),
        "Erro ao atualizar a senha do usuário");
    let verifica_senha_input = VerificaSenhaInput{
        email: email.to_string(),
        senha: nova_senha.to_string()
    };
    let resultado_verificacao 
        = controller::usuario::verifica_senha(Json(verifica_senha_input)).await;
    assert!(resultado_verificacao.is_ok());
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
async fn test_verifica_token_ok(){
    let email = "usuarioteste6@teste.com";
    let nome_completo = "Usuario Teste";        
    let senha = "senhausuarioteste1.";
    let cpf = "12312312301";
    let cnpj = "11331220000101";
    let usuario = estrutura_usuario(&nome_completo, &email, &senha, &cpf, &cnpj);
    assert!(cria_conta(usuario).await.is_ok());
    let idusuario = match _busca_id_usuario(email).await{
        Ok(idusuario) => idusuario,
        Err(e) => {
            println!("Erro ao buscar o ID do usuário: {}", e);
            return;
        }
    };
    let verifica_token_input = VerificaTokenInput{email: email.to_string(), token: idusuario.to_string()};
    assert!(controller::usuario::verifica_token(Json(verifica_token_input)).await.is_ok(),
        "Erro ao verificar o token do usuário");

    assert!(_limpa_usuario(&idusuario, email).await.is_ok(),
        "Erro ao deletar a conta do usuário");
}

#[tokio::test]
async fn test_verifica_token_err(){
    let email = "usuarioteste7@teste.com";
    let nome_completo = "Usuario Teste";        
    let senha = "senhausuarioteste1.";
    let cpf = "12312312301";
    let cnpj = "11331220000101";
    let usuario = estrutura_usuario(&nome_completo, &email, &senha, &cpf, &cnpj);
    assert!(cria_conta(usuario).await.is_ok());
    let idusuario = match _busca_id_usuario(email).await{
        Ok(idusuario) => idusuario,
        Err(e) => {
            println!("Erro ao buscar o ID do usuário: {}", e);
            return;
        }
    };
    let email_2 = "usuarioteste7_2@teste.com";
    let nome_completo_2 = "Usuario Teste";        
    let senha_2 = "senhausuarioteste1.";
    let cpf_2 = "12312312301";
    let cnpj_2 = "11331220000101";
    let usuario = estrutura_usuario(&nome_completo_2, &email_2, &senha_2, &cpf_2, &cnpj_2);
    assert!(cria_conta(usuario).await.is_ok());
    let idusuario_2 = match _busca_id_usuario(email_2).await{
        Ok(idusuario2) => idusuario2,
        Err(e) => {
            println!("Erro ao buscar o ID do usuário: {}", e);
            return;
        }
    };
    let verifica_token_input = VerificaTokenInput{email: email_2.to_string(), token: idusuario.to_string()};
    assert!(controller::usuario::verifica_token(Json(verifica_token_input)).await.is_err(),
        "Erro ao verificar um token inválido do usuário");
    assert!(_limpa_usuario(&idusuario, email).await.is_ok(),
        "Erro ao deletar a conta do usuário");
    assert!(_limpa_usuario(&idusuario_2, email_2).await.is_ok(),
        "Erro ao deletar a conta do usuário");
}

#[tokio::test]
async fn test_busca_nome_usuario(){
    let email = "usuarioteste8@teste.com";
    let nome_completo = "Usuario Teste";        
    let senha = "senhausuarioteste1.";
    let cpf = "12312312301";
    let cnpj = "11331220000101";
    let usuario = estrutura_usuario(&nome_completo, &email, &senha, &cpf, &cnpj);
    assert!(cria_conta(usuario).await.is_ok());
    let idusuario = match _busca_id_usuario(email).await{
        Ok(idusuario) => idusuario,
        Err(e) => {
            println!("Erro ao buscar o ID do usuário: {}", e);
            return;
        }
    };
    let id = idusuario.clone();

    assert!(controller::usuario::busca_nome_usuario(Json(idusuario)).await.is_ok(),
        "Erro ao buscar o nome do usuário");

    assert!(_limpa_usuario(&id, email).await.is_ok(),
        "Erro ao deletar a conta do usuário");
}

#[tokio::test]
async fn test_busca_cnpj_usuario(){
    let email = "usuarioteste9@teste.com";
    let nome_completo = "Usuario Teste";        
    let senha = "senhausuarioteste1.";
    let cpf = "12312312301";
    let cnpj = "11331220000101";
    let usuario = estrutura_usuario(&nome_completo, &email, &senha, &cpf, &cnpj);
    assert!(cria_conta(usuario).await.is_ok());
    let idusuario = match _busca_id_usuario(email).await{
        Ok(idusuario) => idusuario,
        Err(e) => {
            println!("Erro ao buscar o ID do usuário: {}", e);
            return;
        }
    };
    let id = idusuario.clone();

    assert!(controller::usuario::busca_cnpj_usuario(Json(idusuario)).await.is_ok(),
        "Erro ao buscar o CNPJ do usuário");

    assert!(_limpa_usuario(&id, email).await.is_ok(),
        "Erro ao deletar a conta do usuário");
}

#[tokio::test]
async fn test_atualiza_nome(){
    let email = "usuarioteste10@teste.com";
    let nome_completo = "Usuario Teste";        
    let senha = "senhausuarioteste1.";
    let cpf = "12312312301";
    let cnpj = "11331220000101";
    let usuario = estrutura_usuario(&nome_completo, &email, &senha, &cpf, &cnpj);
    assert!(cria_conta(usuario).await.is_ok());
    let idusuario = match _busca_id_usuario(email).await{
        Ok(idusuario) => idusuario,
        Err(e) => {
            println!("Erro ao buscar o ID do usuário: {}", e);
            return;
        }
    };
    let atualiza_nome_input = AtualizaNomeInput{email: email.to_string(), nome: nome_completo.to_string()};
    assert!(controller::usuario::atualiza_nome(Json(atualiza_nome_input)).await.is_ok(), 
        "Erro ao atualizar o nome do usuário");

    assert!(_limpa_usuario(&idusuario, email).await.is_ok(),
        "Erro ao deletar o usuário");
}

