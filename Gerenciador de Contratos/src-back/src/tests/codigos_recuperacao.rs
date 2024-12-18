use axum::Json;

use crate::{controllers::{codigos_recuperacao::{deleta_codigo, envia_codigo_recuperacao, verifica_codigo_recuperacao, CodigoRecuperacaoInput}, cria_conn, usuarios::{cadastra_usuario, realiza_login, CredenciaisUsuario, EmailInput}}, models::{codigos_recuperacao::cadastra_codigo_recuperacao_db, usuarios::deleta_usuario}, tests::usuarios::usuario_padrao};

#[tokio::test]
async fn test_cadastra_codigo_ok(){
    let usuario = usuario_padrao("100");
    let email = usuario.email.clone();

    let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
    let id = usuario.0.clone();

    let idcodigo = envia_codigo_recuperacao(Json(EmailInput{email})).await.unwrap().1.0.0;

    assert!(deleta_codigo(idcodigo).await.is_ok());
    assert!(deleta_usuario(id).await.is_ok());
}

#[tokio::test]
async fn test_cadastra_codigo_err(){
    let usuario = usuario_padrao("101");

    let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
    let id = usuario.0.clone();

    let email_invalido = "emailinvalido@gmail.com".to_string();

    assert!(envia_codigo_recuperacao(Json(EmailInput{email: email_invalido})).await.is_err());
    
    assert!(deleta_usuario(id).await.is_ok());
}

#[tokio::test]
async fn test_verifica_codigo_err(){
    let usuario = usuario_padrao("102");
    let email = usuario.email.to_string();

    let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
    let id = usuario.0.clone();

    let idcodigo = envia_codigo_recuperacao(Json(EmailInput{email})).await.unwrap().1.0.0;

    assert!(verifica_codigo_recuperacao(Json(CodigoRecuperacaoInput{
        idusuario: id.clone(),
        codigodigitado: "CodInvalido".to_string()
    })).await.is_err());

    assert!(deleta_codigo(idcodigo).await.is_ok());
    assert!(deleta_usuario(id).await.is_ok());
}