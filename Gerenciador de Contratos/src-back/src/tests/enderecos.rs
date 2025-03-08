use axum::{extract::Query, Json};

use crate::{controllers::{enderecos::{atualiza_endereco, busca_endereco_id, cadastra_endereco_usuario, EnderecoInput}, enderecos_usuarios::busca_enderecousuario_idusuario, usuarios::{cadastra_usuario, deleta_usuario, IdInput, UserId}}, models::{enderecos::{cadastra_endereco, deleta_endereco}, enderecos_usuarios::deleta_endereco_usuario_idendereco}, tests::{enderecos_usuarios::endereco_usuario_padrao, usuarios::usuario_padrao}};

pub async fn endereco_padrao(numeroteste: &str) -> EnderecoInput{
    let pais = format!("País {}", numeroteste);
    let estado = format!("Estado {}", numeroteste);
    let cidade = format!("Cidade {}", numeroteste);
    let cep = "39606-001".to_string();
    let bairro = format!("Bairro {}", numeroteste);
    let logradouro = format!("Rua {}", numeroteste);
    let numero = format!("Número {}", numeroteste);
    let complemento = format!("Complemento {}", numeroteste);

    EnderecoInput{
        pais,
        estado,
        cidade,
        cep,
        bairro,
        logradouro,
        numero,
        complemento: complemento,
    }
}

#[tokio::test]
pub async fn test_busca_endereco_id_ok(){
    let usuario = usuario_padrao("400");
      
    let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
    let idusuario = usuario.0.idusuario.to_string();

    let endereco = endereco_usuario_padrao(&idusuario, "400").await;

    let id = cadastra_endereco_usuario(Json(endereco)).await.unwrap().1.0.idendereco;
    assert!(busca_endereco_id(Query(id.clone())).await.is_ok());

    assert!(deleta_endereco_usuario_idendereco(id.clone()).await.is_ok());
    assert!(deleta_endereco(id).await.is_ok());
    assert!(deleta_usuario(Query(IdInput{id: idusuario })).await.is_ok());
}

#[tokio::test]
pub async fn test_busca_endereco_id_err(){
    let usuario = usuario_padrao("401");
      
    let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
    let idusuario = usuario.0.idusuario.to_string();

    let endereco = endereco_usuario_padrao(&idusuario, "401").await;

    let id = cadastra_endereco_usuario(Json(endereco)).await.unwrap().1.0.idendereco;
    assert!(busca_endereco_id(Query("id inválido".to_string())).await.is_err());

    assert!(deleta_endereco_usuario_idendereco(id.clone()).await.is_ok());
    assert!(deleta_endereco(id).await.is_ok());
    assert!(deleta_usuario(Query(IdInput{id: idusuario })).await.is_ok());
}

#[tokio::test]
pub async fn test_atualiza_endereco_ok(){
    let usuario = usuario_padrao("402");
      
    let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
    let idusuario = usuario.0.idusuario.to_string();

    let endereco = endereco_usuario_padrao(&idusuario, "402").await;

    let endereco = cadastra_endereco_usuario(Json(endereco)).await.unwrap().1.0;
  
    let mut novo_endereco = endereco;
    novo_endereco.estado = "Estado 402 B".to_string();

    let id = novo_endereco.idendereco.clone();

    assert!(atualiza_endereco(Json(novo_endereco)).await.is_ok());

    assert!(deleta_endereco_usuario_idendereco(id.clone()).await.is_ok());
    assert!(deleta_endereco(id).await.is_ok());
    assert!(deleta_usuario(Query(IdInput{id: idusuario })).await.is_ok());
}

#[tokio::test]
pub async fn test_atualiza_endereco_err(){
    let usuario = usuario_padrao("402");
      
    let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
    let idusuario = usuario.0.idusuario.to_string();

    let endereco = endereco_usuario_padrao(&idusuario, "402").await;

    let endereco = cadastra_endereco_usuario(Json(endereco)).await.unwrap().1.0;
  
    let mut novo_endereco = endereco;
    novo_endereco.estado = "".to_string();

    let id = novo_endereco.idendereco.clone();

    assert!(atualiza_endereco(Json(novo_endereco)).await.is_err());

    assert!(deleta_endereco_usuario_idendereco(id.clone()).await.is_ok());
    assert!(deleta_endereco(id).await.is_ok());
    assert!(deleta_usuario(Query(IdInput{id: idusuario })).await.is_ok());
}