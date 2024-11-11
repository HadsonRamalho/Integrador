use axum::Json;

use crate::{controllers::usuarios::{busca_email_usuario, cadastra_usuario, estrutura_usuario, valida_email, valida_senha, EmailInput, UsuarioInput}, models::usuarios::deleta_usuario};

#[tokio::test]
async fn test_estrutura_usuario_ok(){
    let email = "testeunit1@user.com".to_string();
    let nome = "Usuario Teste 1".to_string();
    let senha = "senhateste1.".to_string();
    let documento = "113.123.113-10".to_string();

    let usuario = UsuarioInput{
        nome,
        email,
        senha,
        documento
    };

    assert!(estrutura_usuario(Json(usuario)).await.is_ok());
}

#[tokio::test]
async fn test_cadastra_usuario_ok(){
    let email = "testeunit2@gmail.com".to_string();
    let nome = "Usuario Teste 2".to_string();
    let senha = "senhateste2.".to_string();
    let documento = "002.123.113-10".to_string();

    let usuario = UsuarioInput{
        nome,
        email,
        senha,
        documento
    };

    let usuario = estrutura_usuario(Json(usuario)).await.unwrap().1;
    let id = usuario.idusuario.clone();
    assert!(cadastra_usuario(usuario).await.is_ok());

    assert!(deleta_usuario(id).await.is_ok());
}

#[tokio::test]
async fn test_busca_email_usuario_ok(){
    let email = "testeunit3@gmail.com".to_string();
    let nome = "Usuario Teste 3".to_string();
    let senha = "senhateste3.".to_string();
    let documento = "003.123.113-10".to_string();

    let usuario = UsuarioInput{
        nome,
        email,
        senha,
        documento
    };

    let usuario = estrutura_usuario(Json(usuario)).await.unwrap().1;
    let id = usuario.idusuario.clone();
    assert!(cadastra_usuario(usuario).await.is_ok());
    assert!(busca_email_usuario(Json(id.clone())).await.is_ok());

    assert!(deleta_usuario(id).await.is_ok());
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