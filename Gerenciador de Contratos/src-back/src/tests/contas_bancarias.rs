use axum::{extract::Query, Json};

use crate::{controllers::{contas_bancarias::{cadastra_conta_bancaria, ContaBancariaInput}, usuarios::{cadastra_usuario, deleta_usuario, IdInput}}, models::contas_bancarias::deleta_conta_bancaria, tests::usuarios::usuario_padrao};


pub fn conta_bancaria_padrao(idusuario: &str, numeroteste: &str) -> ContaBancariaInput{
    let numeroconta = format!("120{}045", numeroteste);
    let numeroagencia = format!("0{}", numeroteste);
    let nomebanco = format!("Banco Teste {}", numeroteste);
    ContaBancariaInput{
      idusuario: idusuario.to_string(),
      numeroagencia,
      numeroconta,
      nomebanco
    }
}

#[tokio::test]
async fn test_cadastra_conta_bancaria_ok(){
  let usuario = usuario_padrao("600");

  let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
  let idusuario = usuario.0.idusuario.to_string();

  let conta = conta_bancaria_padrao(&idusuario, "600");

  let idconta = cadastra_conta_bancaria(Json(conta)).await.unwrap().1.0.to_string();  

  assert!(deleta_usuario(Query(IdInput{id: idusuario})).await.is_ok());
  assert!(deleta_conta_bancaria(idconta).await.is_ok());
}

#[tokio::test]
async fn test_cadastra_conta_bancaria_err(){
  let usuario = usuario_padrao("601");

  let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
  let idusuario = usuario.0.idusuario.to_string();

  assert!(cadastra_conta_bancaria(Json(
    ContaBancariaInput{ 
      idusuario: idusuario.clone(),
      numeroconta: "".to_string(), 
      numeroagencia: "".to_string(), 
      nomebanco: "".to_string() }
  )).await.is_err());

  assert!(deleta_usuario(Query(IdInput{id: idusuario})).await.is_ok());
}

