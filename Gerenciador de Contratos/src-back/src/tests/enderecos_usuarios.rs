use axum::{extract::Query, Json};

use crate::{controllers::{enderecos::{cadastra_endereco_usuario, EnderecoUsuarioInput}, enderecos_usuarios::busca_enderecousuario_idusuario, usuarios::{cadastra_usuario, deleta_usuario, IdInput, UserId}}, models::{enderecos::deleta_endereco, enderecos_usuarios::deleta_endereco_usuario_idendereco}, tests::usuarios::usuario_padrao};

pub async fn endereco_usuario_padrao(idusuario: &str, numeroteste: &str) -> EnderecoUsuarioInput{
    let idusuario = idusuario.to_string();
    let pais = format!("País {}", numeroteste);
    let estado = format!("Estado {}", numeroteste);
    let cidade = format!("Cidade {}", numeroteste);
    let cep = "39606-001".to_string();
    let bairro = format!("Bairro {}", numeroteste);
    let logradouro = format!("Rua {}", numeroteste);
    let numero = format!("Número {}", numeroteste);
    let complemento = format!("Complemento {}", numeroteste);
    EnderecoUsuarioInput{
        idusuario,
        pais,
        estado,
        cidade,
        cep,
        bairro,
        logradouro,
        numero,
        complemento: Some(complemento),
    }
}

#[tokio::test]
pub async fn test_cadastra_endereco_usuario_ok(){
    let usuario = usuario_padrao("300");
    
    let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
    let idusuario = usuario.0.idusuario.to_string();

    let endereco = endereco_usuario_padrao(&idusuario, "300").await;

    let id = cadastra_endereco_usuario(Json(endereco)).await.unwrap().1.0.idendereco;

    assert!(deleta_endereco_usuario_idendereco(id.clone()).await.is_ok());
    assert!(deleta_endereco(id).await.is_ok());
    assert!(deleta_usuario(Query(IdInput{id: idusuario })).await.is_ok());
}

#[tokio::test]
pub async fn test_cadastra_endereco_usuario_err(){
    let usuario = usuario_padrao("301");

    let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
    let idusuario = usuario.0.idusuario.to_string();

    let mut endereco = endereco_usuario_padrao(&idusuario, "301").await;

    endereco.cep = "".to_string();
    endereco.cidade = "".to_string();

    assert!(cadastra_endereco_usuario(Json(endereco)).await.is_err());

    assert!(deleta_usuario(Query(IdInput{id: idusuario })).await.is_ok());
}

#[tokio::test]
pub async fn test_busca_endereco_usuario_idusuario_ok(){
    let usuario = usuario_padrao("302");
      
    let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
    let idusuario = usuario.0.idusuario.to_string();

    let endereco = endereco_usuario_padrao(&idusuario, "302").await;

    let id = cadastra_endereco_usuario(Json(endereco)).await.unwrap().1.0.idendereco;
    assert!(busca_enderecousuario_idusuario(Query(UserId{idusuario: idusuario.clone()})).await.is_ok());

    assert!(deleta_endereco_usuario_idendereco(id.clone()).await.is_ok());
    assert!(deleta_endereco(id).await.is_ok());
    assert!(deleta_usuario(Query(IdInput{id: idusuario })).await.is_ok());
}