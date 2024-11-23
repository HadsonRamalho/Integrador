use axum::Json;
use pwhash::bcrypt::verify;

use crate::{controllers::usuarios::{atualiza_email_usuario, atualiza_senha_usuario, busca_email_usuario, busca_usuario_email, cadastra_usuario, estrutura_usuario, formata_documento, realiza_login, valida_email, valida_senha, AtualizaEmailInput, AtualizaSenhaInput, CredenciaisUsuario, EmailInput, UsuarioInput}, models::usuarios::{busca_senha_usuario, deleta_usuario, Usuario}};

pub fn usuario_padrao(numeroteste: &str) -> UsuarioInput{
    let email = format!("testeunit{}@gmail.com", numeroteste);
    let nome = format!("Usuario Teste {}", numeroteste);
    let senha = format!("SenhaTeste{}.", numeroteste);
    let documento = format!("{}.123.113-10", numeroteste);

    UsuarioInput{
        email,
        nome,
        senha,
        documento
    }
}

#[tokio::test]
async fn test_estrutura_usuario_ok(){
    let usuario = usuario_padrao("001");

    assert!(estrutura_usuario(Json(usuario)).await.is_ok());
}

#[tokio::test]
async fn test_cadastra_usuario_ok(){
    let usuario = usuario_padrao("002");

    let usuario = estrutura_usuario(Json(usuario)).await.unwrap().1;
    let id = usuario.idusuario.clone();
    assert!(cadastra_usuario(usuario).await.is_ok());

    assert!(deleta_usuario(id).await.is_ok());
}

#[tokio::test]
async fn test_cadastra_usuario_err(){
    let email_invalido = "@gmail.com".to_string();
    let nome = "Usuario Teste 2".to_string();
    let senha = "Senhateste2.".to_string();
    let documento = "002.123.113-10".to_string();
    let now = chrono::Utc::now().naive_utc();
    let usuario = Usuario{
        email: email_invalido,
        nome,
        senha,
        documento,
        datacadastro: now,
        idusuario: "123145123".to_string()
    };
    let id = usuario.idusuario.clone();
    assert!(cadastra_usuario(Json(usuario)).await.is_err());

    assert!(deleta_usuario(id).await.is_err());
}

#[tokio::test]
async fn test_busca_email_usuario_ok(){
    let usuario = usuario_padrao("003");

    let usuario = estrutura_usuario(Json(usuario)).await.unwrap().1;
    let id = usuario.idusuario.clone();
    assert!(cadastra_usuario(usuario).await.is_ok());
    assert!(busca_email_usuario(Json(id.clone())).await.is_ok());

    assert!(deleta_usuario(id).await.is_ok());
}

#[tokio::test]
async fn test_busca_email_usuario_err(){
    let id = "123456789";
    assert!(busca_email_usuario(Json(id.to_string())).await.is_err());
}

#[tokio::test]
async fn test_valida_senha_ok(){
    let senha1 = "SenhaForte01_";
    assert!(valida_senha(senha1).is_ok());
    
    let senha2 = ".senha0fortE";
    assert!(valida_senha(senha2).is_ok());
}

#[tokio::test]
async fn test_valida_senha_err(){
    let senha1 = "senhafraca1";
    assert!(valida_senha(senha1).is_err());

    let senha2 = "SENHAFRACA2";
    assert!(valida_senha(senha2).is_err());

    let senha3 = "12345678";
    assert!(valida_senha(senha3).is_err());

    let senha4 = "abcd";
    assert!(valida_senha(senha4).is_err());

    let senha5 = ".-=+_+.@==.!@";
    assert!(valida_senha(senha5).is_err());
}

#[tokio::test]
async fn test_valida_email_ok(){
    let email1 = "test@test.com";
    assert!(valida_email(Json({
        EmailInput{
            email: email1.to_string()
        }
    })).await.is_ok());

    let email2 = "test2@t.com";
    assert!(valida_email(Json({
        EmailInput{
            email: email2.to_string()
        }
    })).await.is_ok());

    let email3 = "t@t.c";
    assert!(valida_email(Json({
        EmailInput{
            email: email3.to_string()
        }
    })).await.is_ok());
}

#[tokio::test]
async fn test_valida_email_err(){
    let email1 = "e@.";
    assert!(valida_email(Json({
        EmailInput{
            email: email1.to_string()
        }
    })).await.is_err());

    let email2 = "email@";
    assert!(valida_email(Json({
        EmailInput{
            email: email2.to_string()
        }
    })).await.is_err());

    let email3 = "@email.c";
    assert!(valida_email(Json({
        EmailInput{
            email: email3.to_string()
        }
    })).await.is_err());

    
    let email4 = "email com espaco @ email . com";
    assert!(valida_email(Json(
        EmailInput{
            email: email4.to_string()
        }
    )).await.is_err());
}

#[tokio::test]
async fn test_formata_documento_ok(){
    let doc1 = "11.123.123/0001-01";
    assert!(formata_documento(doc1).is_ok());

    let doc2 = "11.421a213x0001p01";
    assert!(formata_documento(doc2).is_ok());

    let doc3 = "11421123000101";
    assert!(formata_documento(doc3).is_ok());

    let doc4 = "123.456.789-01";
    assert!(formata_documento(doc4).is_ok());

    let doc5 = "123x456xyz789x01";
    assert!(formata_documento(doc5).is_ok());

    let doc6 = "12345678901";
    assert!(formata_documento(doc6).is_ok());
}

#[tokio::test]
async fn test_formata_documento_err(){
    let doc1 = "11.123.123/0001-";
    assert!(formata_documento(doc1).is_err());

    let doc2 = "123.412.131-";
    assert!(formata_documento(doc2).is_err());

    let doc3 = "123x123xxxxxxx";
    assert!(formata_documento(doc3).is_err());

    let doc4 = "11.123.123/0001-011111";
    assert!(formata_documento(doc4).is_err());
}

#[tokio::test]
async fn test_atualiza_email_usuario_ok(){
    let usuario = usuario_padrao("004");
    let email = usuario.email.clone();
    let senha = usuario.senha.clone();

    let email_novo = "xtesteunit4@gmail.comx".to_string();

    let usuario = estrutura_usuario(Json(usuario)).await.unwrap().1;
    let id = usuario.idusuario.clone();
    assert!(cadastra_usuario(usuario).await.is_ok());
    assert!(atualiza_email_usuario(Json(AtualizaEmailInput{
        email_antigo: email,
        email_novo,
        senha
    })).await.is_ok());

    assert!(deleta_usuario(id).await.is_ok());
}

#[tokio::test]
async fn test_atualiza_email_usuario_err(){
    let email1 = "testeunit5@gmail.com".to_string();
    let nome = "Usuario Teste 5".to_string();
    let senha = "Senhateste5.".to_string();
    let documento = "005.123.113-10".to_string();

    let usuario = UsuarioInput{
        nome,
        email: email1.clone(),
        senha,
        documento
    };

    let usuario = estrutura_usuario(Json(usuario)).await.unwrap().1;
    let id1 = usuario.idusuario.clone();
    assert!(cadastra_usuario(usuario).await.is_ok());

    let email2 = "testeunit6@gmail.com".to_string();
    let nome = "Usuario Teste 6".to_string();
    let senha = "Senhateste6.".to_string();
    let documento = "006.123.113-10".to_string();

    let usuario = UsuarioInput{
        nome,
        email: email2.clone(),
        senha: senha.clone(),
        documento
    };

    let usuario = estrutura_usuario(Json(usuario)).await.unwrap().1;
    let id2 = usuario.idusuario.clone();
    assert!(cadastra_usuario(usuario).await.is_ok());

    assert!(atualiza_email_usuario(Json(AtualizaEmailInput{
        email_antigo: email1,
        email_novo: email2,
        senha
    })).await.is_err());

    assert!(deleta_usuario(id1).await.is_ok());
    assert!(deleta_usuario(id2).await.is_ok())
}

#[tokio::test]
pub async fn busca_usuario_email_ok(){
    let usuario = usuario_padrao("007");
    let email = usuario.email.clone();

    let usuario = estrutura_usuario(Json(usuario)).await.unwrap().1;
    let id = usuario.idusuario.clone();
    assert!(cadastra_usuario(usuario).await.is_ok());
    
    assert!(busca_usuario_email(Json(
        EmailInput{
            email
        }
    )).await.is_ok());

    assert!(deleta_usuario(id).await.is_ok());
}

#[tokio::test]
pub async fn busca_usuario_email_err(){
    let usuario = usuario_padrao("008");

    let usuario = estrutura_usuario(Json(usuario)).await.unwrap().1;
    let id = usuario.idusuario.clone();
    assert!(cadastra_usuario(usuario).await.is_ok());
    assert!(busca_usuario_email(Json(
        EmailInput{
            email: "emailinvalido1@gmail.com".to_string()
        }
    )).await.is_err());

    assert!(deleta_usuario(id).await.is_ok());
}

#[tokio::test]
async fn test_busca_senha_usuario_ok(){
    let usuario = usuario_padrao("009");
    let email = usuario.email.clone();
    let senha = usuario.senha.clone();

    let usuario = estrutura_usuario(Json(usuario)).await.unwrap().1;
    let id = usuario.idusuario.clone();
    assert!(cadastra_usuario(usuario).await.is_ok());
    let hash = busca_senha_usuario(email).await.unwrap();
    assert!(verify(senha, &hash));

    assert!(deleta_usuario(id).await.is_ok());
}

#[tokio::test]
async fn test_busca_senha_usuario_err(){
    let usuario = usuario_padrao("010");
    let email = usuario.email.clone();

    let usuario = estrutura_usuario(Json(usuario)).await.unwrap().1;
    let id = usuario.idusuario.clone();

    assert!(cadastra_usuario(usuario).await.is_ok());

    let hash = busca_senha_usuario(email).await.unwrap();
    let senha_incorreta = "SenhaIncorreta1.";
    assert!(!verify(senha_incorreta, &hash));

    assert!(deleta_usuario(id).await.is_ok());
}

#[tokio::test]
async fn test_realiza_login_ok(){
    let usuario = usuario_padrao("011");
    let email = usuario.email.clone();
    let senha = usuario.senha.clone();

    let usuario = estrutura_usuario(Json(usuario)).await.unwrap().1;
    let id = usuario.idusuario.clone();

    assert!(cadastra_usuario(usuario).await.is_ok());

    assert!(realiza_login(Json(
        CredenciaisUsuario{
            email,
            senha
        }
    )).await.is_ok());

    assert!(deleta_usuario(id).await.is_ok());
}

#[tokio::test]
async fn test_realiza_login_err(){
    let usuario = usuario_padrao("012");
    let email = usuario.email.clone();
    let senha = usuario.senha.clone();

    let usuario = estrutura_usuario(Json(usuario)).await.unwrap().1;
    let id = usuario.idusuario.clone();

    assert!(cadastra_usuario(usuario).await.is_ok());

    assert!(realiza_login(Json(
        CredenciaisUsuario{
            email,
            senha
        }
    )).await.is_ok());

    assert!(deleta_usuario(id).await.is_ok());
}

#[tokio::test]
async fn test_atualiza_senha_usuario_ok(){
    let usuario = usuario_padrao("013");
    let email = usuario.email.clone();
    let senha = usuario.senha.clone();

    let usuario = estrutura_usuario(Json(usuario)).await.unwrap().1;
    let id = usuario.idusuario.clone();

    assert!(cadastra_usuario(usuario).await.is_ok());

    assert!(realiza_login(Json(
        CredenciaisUsuario{
            email: email.clone(),
            senha: senha.clone()
        }
    )).await.is_ok());

    let senha_nova = "SenhaTeste13.Nova".to_string();

    assert!(atualiza_senha_usuario(Json(AtualizaSenhaInput{
        email: email.clone(),
        senha_antiga: senha,
        senha_nova: senha_nova.clone()
    })).await.is_ok());

    assert!(realiza_login(Json(CredenciaisUsuario{
        email: email,
        senha: senha_nova
    })).await.is_ok());

    assert!(deleta_usuario(id).await.is_ok());
}

#[tokio::test]
async fn test_atualiza_senha_usuario_err(){
    let usuario = usuario_padrao("014");
    let email = usuario.email.clone();
    let senha = usuario.senha.clone();

    let usuario = estrutura_usuario(Json(usuario)).await.unwrap().1;
    let id = usuario.idusuario.clone();

    assert!(cadastra_usuario(usuario).await.is_ok());

    assert!(realiza_login(Json(
        CredenciaisUsuario{
            email: email.clone(),
            senha: senha.clone()
        }
    )).await.is_ok());

    let senha_nova = "SenhaTeste14.Nova".to_string();
    let email_invalido = "email_invalido@gmai.com".to_string();

    assert!(atualiza_senha_usuario(Json(AtualizaSenhaInput{
        email: email_invalido,
        senha_antiga: senha.clone(),
        senha_nova: senha_nova.clone()
    })).await.is_err());

    let senha_invalida = "senhainvalida".to_string();
    assert!(atualiza_senha_usuario(Json(AtualizaSenhaInput{
        email: email.clone(),
        senha_antiga: senha.clone(),
        senha_nova: senha_invalida
    })).await.is_err());

    assert!(realiza_login(Json(CredenciaisUsuario{
        email: email,
        senha: senha_nova
    })).await.is_err());

    assert!(deleta_usuario(id).await.is_ok());
}