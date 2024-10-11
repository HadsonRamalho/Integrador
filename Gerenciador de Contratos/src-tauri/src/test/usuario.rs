use axum::Json;
use mysql_async::Pool;

use crate::{controller::{self, cria_pool, usuario::{self, atualiza_email, atualiza_senha, cria_conta, deleta_conta, verifica_senha, MyResult, UsuarioInput}}, model::{self, usuario::busca_id_usuario}};
#[cfg(test)]

async fn cria_usuario_teste(nome: &str, email: &str, senha: &str, cpf: &str, cnpj: &str) -> controller::usuario::MyResult {
    use axum::Json;

    use crate::controller::usuario::{MyResult, UsuarioInput};
    let usuario = UsuarioInput{
        nome: nome.to_string(),
        email: email.to_string(),
        senha1: senha.to_string(),
        senha2: senha.to_string(),
        cpf: cpf.to_string(),
        cnpj: cnpj.to_string(),
    };
    let usuario = Json(usuario);
    controller::usuario::cria_conta(usuario).await
}

fn estrutura_usuario(nome: &str, email: &str, senha: &str, cpf: &str, cnpj: &str) -> Json<UsuarioInput> {
    use axum::Json;

    use crate::controller::usuario::{MyResult, UsuarioInput};
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
    let email = "usuarioteste1@teste.com";
    let nome_completo = "Usuario Teste";        
    let senha = "senhausuarioteste1.";
    let cpf = "12312312301";
    let cnpj = "11331220000101";
    match cria_usuario_teste(nome_completo, email, senha, cpf, cnpj).await{
        MyResult::Success(_) => {},
        MyResult::Error(_) => {
            println!("Erro ao criar a conta do usuario");
            return;
        }
    }
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
    match cria_usuario_teste(nome_completo, email, senha, cpf, cnpj).await{
        MyResult::Success(_) => {},
        MyResult::Error(_) => {
            println!("Erro ao criar a conta do usuario");
            return;
        }
    }

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
    match cria_conta(usuario).await{
        MyResult::Success(_) => {},
        MyResult::Error(e) => {
            println!("{:?}", e);
            return;
        } 
    }
    let resultado_verificacao = controller::usuario::verifica_senha(email, senha).await;
    match resultado_verificacao{
        MyResult::Success(_) => {},
        MyResult::Error(e) => {
            println!("{:?}", e);
            return;
        }
    }
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
    match cria_conta(usuario).await{
        MyResult::Success(_) => {},
        MyResult::Error(e) => {
            println!("{:?}", e);
            return;
        } 
    }

    let pool = match _setup_pool().await{
        Ok(pool) => {pool},
        Err(e) => {
            println!("Erro ao criar a pool: {}", e);
            return;
        }
    };
    let novo_email = "usuariotesteA@teste.com";
    
    assert!(model::busca_email(&pool, novo_email).await.is_err(),
        "Erro ao buscar e-mail existente");

    assert!(atualiza_email(email.to_string(), novo_email.to_string()).await.is_ok(),
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
    match cria_conta(usuario).await{
        MyResult::Success(_) => {},
        MyResult::Error(e) => {
            println!("{:?}", e);
            return;
        } 
    }
    let nova_senha = "novasenhausuarioteste1.";
    assert!(atualiza_senha(email, nova_senha).await.is_ok(),
        "Erro ao atualizar a senha do usuário");

    let resultado_verificacao = controller::usuario::verifica_senha(email, &nova_senha).await;
    match resultado_verificacao{
        MyResult::Success(_) => {},
        MyResult::Error(e) => {
            println!("{:?}", e);
            return;
        }
    }
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
    match cria_conta(usuario).await{
        MyResult::Success(_) => {},
        MyResult::Error(e) => {
            println!("{:?}", e);
            return;
        } 
    }
    let idusuario = match _busca_id_usuario(email).await{
        Ok(idusuario) => idusuario,
        Err(e) => {
            println!("Erro ao buscar o ID do usuário: {}", e);
            return;
        }
    };

    assert!(controller::usuario::verifica_token(email, &idusuario).await.is_ok(),
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
    match cria_conta(usuario).await{
        MyResult::Success(_) => {},
        MyResult::Error(e) => {
            println!("{:?}", e);
            return;
        } 
    }
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
    match cria_conta(usuario).await{
        MyResult::Success(_) => {},
        MyResult::Error(e) => {
            println!("{:?}", e);
            return;
        } 
    }
    let idusuario_2 = match _busca_id_usuario(email_2).await{
        Ok(idusuario2) => idusuario2,
        Err(e) => {
            println!("Erro ao buscar o ID do usuário: {}", e);
            return;
        }
    };
    assert!(controller::usuario::verifica_token(email_2, &idusuario).await.is_err(),
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
    match cria_conta(usuario).await{
        MyResult::Success(_) => {},
        MyResult::Error(e) => {
            println!("{:?}", e);
            return;
        } 
    }
    let idusuario = match _busca_id_usuario(email).await{
        Ok(idusuario) => idusuario,
        Err(e) => {
            println!("Erro ao buscar o ID do usuário: {}", e);
            return;
        }
    };
    let id = idusuario.clone();

    assert!(controller::usuario::busca_nome_usuario(idusuario).await.is_ok(),
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
    match cria_conta(usuario).await{
        MyResult::Success(_) => {},
        MyResult::Error(e) => {
            println!("{:?}", e);
            return;
        } 
    }
    let idusuario = match _busca_id_usuario(email).await{
        Ok(idusuario) => idusuario,
        Err(e) => {
            println!("Erro ao buscar o ID do usuário: {}", e);
            return;
        }
    };
    let id = idusuario.clone();

    assert!(controller::usuario::busca_cnpj_usuario(idusuario).await.is_ok(),
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
    match cria_conta(usuario).await{
        MyResult::Success(_) => {},
        MyResult::Error(e) => {
            println!("{:?}", e);
            return;
        } 
    }
    let idusuario = match _busca_id_usuario(email).await{
        Ok(idusuario) => idusuario,
        Err(e) => {
            println!("Erro ao buscar o ID do usuário: {}", e);
            return;
        }
    };

    assert!(controller::usuario::atualiza_nome(email, nome_completo).await.is_ok(), 
        "Erro ao atualizar o nome do usuário");

    assert!(_limpa_usuario(&idusuario, email).await.is_ok(),
        "Erro ao deletar o usuário");
}

