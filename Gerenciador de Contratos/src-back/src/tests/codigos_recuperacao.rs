use axum::Json;

use crate::{controllers::{codigos_recuperacao::{deleta_codigo, envia_codigo_recuperacao}, cria_conn, usuarios::{cadastra_usuario, estrutura_usuario, realiza_login, CredenciaisUsuario, EmailInput}}, models::{codigos_recuperacao::cadastra_codigo_recuperacao_db, usuarios::deleta_usuario}, tests::usuarios::usuario_padrao};

#[tokio::test]
async fn test_cadastra_codigo_ok(){
    let usuario = usuario_padrao("100");
    let email = usuario.email.clone();

    let usuario = estrutura_usuario(Json(usuario)).await.unwrap().1;
    let id = usuario.idusuario.clone();

    assert!(cadastra_usuario(usuario).await.is_ok());

    let idcodigo = envia_codigo_recuperacao(Json(EmailInput{email})).await.unwrap().1.0;

    assert!(deleta_codigo(idcodigo).await.is_ok());
    assert!(deleta_usuario(id).await.is_ok());
}