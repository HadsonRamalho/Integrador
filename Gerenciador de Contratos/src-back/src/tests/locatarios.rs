use axum::{extract::Query, Json};

use crate::{controllers::{enderecos::{cadastra_endereco_usuario, EnderecoUsuarioInput}, locatarios::{busca_locatario_idusuario, cadastra_locatario, LocatarioInput}, usuarios::{cadastra_usuario, deleta_usuario, IdInput}}, models::{enderecos::deleta_endereco, locatarios::deleta_locatario}, tests::{enderecos::endereco_padrao, enderecos_usuarios::endereco_usuario_padrao, usuarios::usuario_padrao}};

pub fn locatario_padrao(idusuario: String, idendereco: String) -> LocatarioInput{
    LocatarioInput{
        idusuario,
        idendereco,
    }
}

#[tokio::test]
async fn test_cadastra_locatario_ok(){
    let usuario = usuario_padrao("500");

    let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
    let idusuario = usuario.0.idusuario.to_string();

    let endereco = endereco_usuario_padrao(&idusuario, "500").await;
    let idendereco = cadastra_endereco_usuario(Json(endereco)).await.unwrap().1.0.idendereco;

    let locatario = locatario_padrao(idusuario.clone(), idendereco.clone());
    let idlocatario = cadastra_locatario(Json(locatario)).await.unwrap().1.0;

    assert!(deleta_locatario(idlocatario).await.is_ok());
    assert!(deleta_endereco(idendereco).await.is_ok());
    assert!(deleta_usuario(Query(IdInput{id: idusuario})).await.is_ok());
}

#[tokio::test]
async fn test_cadastra_locatario_err(){
    assert!(cadastra_locatario(Json(LocatarioInput{
      idendereco: "".to_string(),
      idusuario: "ID inválido".to_string()
    })).await.is_err());
}

#[tokio::test]
async fn test_busca_locatario_idusuario_ok(){
  let usuario = usuario_padrao("502");

  let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
  let idusuario = usuario.0.idusuario.to_string();

  let endereco = endereco_usuario_padrao(&idusuario, "502").await;
  let idendereco = cadastra_endereco_usuario(Json(endereco)).await.unwrap().1.0.idendereco;

  let locatario = locatario_padrao(idusuario.clone(), idendereco.clone());
  let idlocatario = cadastra_locatario(Json(locatario)).await.unwrap().1.0;

  assert!(busca_locatario_idusuario(Query(IdInput{id: idusuario.clone()})).await.is_ok());

  assert!(deleta_locatario(idlocatario).await.is_ok());
  assert!(deleta_endereco(idendereco).await.is_ok());
  assert!(deleta_usuario(Query(IdInput{id: idusuario})).await.is_ok());
}
