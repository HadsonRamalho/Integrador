use axum::{extract::Query, Json};

use crate::{controllers::{contas_bancarias::cadastra_conta_bancaria, enderecos::cadastra_endereco_usuario, locadoras::{cadastra_locadora, LocadoraInput}, usuarios::{cadastra_usuario, deleta_usuario, IdInput}}, models::{contas_bancarias::deleta_conta_bancaria, enderecos::deleta_endereco, locadoras::deleta_locadora}, tests::{contas_bancarias::conta_bancaria_padrao, enderecos_usuarios::endereco_usuario_padrao, usuarios::usuario_padrao}};

pub fn locadora_padrao(idusuario: String, idendereco: String, idconta: String) -> LocadoraInput{
    LocadoraInput{
        idusuario,
        idendereco,
        idconta
    }
}

#[tokio::test]
async fn test_cadastra_locadora_ok(){
    let usuario = usuario_padrao("700");

    let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
    let idusuario = usuario.0.idusuario.to_string();

    let endereco = endereco_usuario_padrao(&idusuario, "700").await;
    let idendereco = cadastra_endereco_usuario(Json(endereco)).await.unwrap().1.0.idendereco;
    
    let conta = conta_bancaria_padrao(&idusuario, "700");
    let idconta = cadastra_conta_bancaria(Json(conta)).await.unwrap().1.0;

    let locadora = locadora_padrao(idusuario.clone(), idendereco.clone(), idconta.clone());
    let idlocadora = cadastra_locadora(Json(locadora)).await.unwrap().1.0;

    assert!(deleta_locadora(idlocadora).await.is_ok());
    assert!(deleta_endereco(idendereco).await.is_ok());
    assert!(deleta_conta_bancaria(idconta).await.is_ok());
    assert!(deleta_usuario(Query(IdInput{id: idusuario})).await.is_ok());
}