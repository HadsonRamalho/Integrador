use axum::Json;

use crate::controllers::usuarios::{cadastra_usuario, estrutura_usuario, UsuarioInput};

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
    let res = cadastra_usuario(usuario).await.unwrap();
    println!("{:?}", res);

}