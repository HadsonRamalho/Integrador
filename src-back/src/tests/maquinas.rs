use axum::{extract::Query, Json};

use crate::{controllers::{maquinas::{busca_maquina_id, cadastra_maquina, deleta_maquina_id, lista_todas_maquinas, MaquinaInput}, maquinas_usuarios::busca_maquinas_usuario_idusuario, usuarios::{cadastra_usuario, IdInput}}, models::usuarios::deleta_usuario, tests::usuarios::usuario_padrao};

pub struct MaquinaInputTeste{
    pub nome: String,
    pub numeroserie: String,
    pub valoraluguel: f64,
    pub disponivelaluguel: String,
    pub status: String,
    pub categoria: String,
    pub descricao: String
}


pub async fn maquina_padrao(numeroteste: &str) -> MaquinaInputTeste{
    let nome = format!("Maquina Teste {}", numeroteste);
    let numeroserie = format!("TEST-NS{}", numeroteste);
    let valoraluguel: f64 = numeroteste.parse().unwrap_or(169.1);
    let disponivelaluguel = "Sim".to_string();
    let status = "Ativo".to_string();
    let descricao =  format!("Descrição N{}", numeroteste);
    let categoria = "Máquina de Teste".to_string();

    MaquinaInputTeste{
        nome,
        numeroserie,
        valoraluguel,
        disponivelaluguel,
        status,
        descricao,
        categoria
    }
}

pub async fn converte_tipo_maquina(maq: MaquinaInputTeste, id: String)
    -> MaquinaInput{
    MaquinaInput{
        idusuario: id,
        nome: maq.nome,
        numeroserie: maq.numeroserie,
        valoraluguel: maq.valoraluguel,
        disponivelaluguel: maq.disponivelaluguel,
        status: maq.status,
        categoria: maq.categoria,
        descricao: maq.descricao
    }
}

#[tokio::test]
async fn test_cadastra_maquina_ok(){
    let maquina = maquina_padrao("201").await;

    let usuario = usuario_padrao("201");

    let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
    let idusuario = usuario.0.idusuario.to_string();

    let idsmaquina = cadastra_maquina(Json(
        converte_tipo_maquina(maquina, idusuario.clone()
        ).await)).await.unwrap().1;
    let id = idsmaquina.0.idmaquina.to_string();

    assert!(deleta_usuario(idusuario).await.is_ok());
    assert!(deleta_maquina_id(id).await.is_ok());
}

#[tokio::test]
async fn test_lista_todas_maquinas_ok(){
    let maquina = maquina_padrao("202").await;

    let usuario = usuario_padrao("202");

    let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
    let idusuario = usuario.0.idusuario.to_string();

    let idsmaquina = cadastra_maquina(Json(
        converte_tipo_maquina(maquina, idusuario.clone()
        ).await)).await.unwrap().1;
    let id = idsmaquina.0.idmaquina.to_string();

    assert!(lista_todas_maquinas().await.is_ok());
    
    assert!(deleta_usuario(idusuario).await.is_ok());
    assert!(deleta_maquina_id(id).await.is_ok());
}

#[tokio::test]
async fn busca_maquina_id_ok(){
    let maquina = maquina_padrao("203").await;

    let usuario = usuario_padrao("203");

    let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
    let idusuario = usuario.0.idusuario.to_string();

    let idsmaquina = cadastra_maquina(Json(
        converte_tipo_maquina(maquina, idusuario.clone()
        ).await)).await.unwrap().1;
    let id = idsmaquina.0.idmaquina.to_string();

    assert!(busca_maquina_id(Query(IdInput{id: id.clone()})).await.is_ok());
    
    assert!(deleta_usuario(idusuario).await.is_ok());
    assert!(deleta_maquina_id(id).await.is_ok());
}

#[tokio::test]
async fn busca_maquina_id_err(){
    let maquina = maquina_padrao("204").await;

    let usuario = usuario_padrao("204");

    let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
    let idusuario = usuario.0.idusuario.to_string();

    let idsmaquina = cadastra_maquina(Json(
        converte_tipo_maquina(maquina, idusuario.clone()
        ).await)).await.unwrap().1;
    let id = idsmaquina.0.idmaquina.to_string();

    assert!(busca_maquina_id(Query(IdInput{id: "idinvalido".to_string()})).await.is_err());
    
    assert!(deleta_usuario(idusuario).await.is_ok());
    assert!(deleta_maquina_id(id).await.is_ok());
}

#[tokio::test]
async fn busca_maquinas_usuario_idusuario_ok(){
    let maquina = maquina_padrao("205").await;

    let usuario = usuario_padrao("205");

    let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
    let idusuario = usuario.0.idusuario.to_string();

    let idsmaquina = cadastra_maquina(Json(
        converte_tipo_maquina(maquina, idusuario.clone()
        ).await)).await.unwrap().1;
    let id = idsmaquina.0.idmaquina.to_string();

    assert!(busca_maquinas_usuario_idusuario(Query(IdInput{
        id: idusuario.clone()
    })).await.is_ok());
    
    assert!(deleta_usuario(idusuario).await.is_ok());
    assert!(deleta_maquina_id(id).await.is_ok());
}

#[tokio::test]
async fn busca_maquinas_usuario_idusuario_err(){
    let maquina = maquina_padrao("206").await;

    let usuario = usuario_padrao("206");

    let usuario = cadastra_usuario(Json(usuario)).await.unwrap().1;
    let idusuario = usuario.0.idusuario.to_string();

    let idsmaquina = cadastra_maquina(Json(
        converte_tipo_maquina(maquina, idusuario.clone()
        ).await)).await.unwrap().1;
    let id = idsmaquina.0.idmaquina.to_string();

    assert!(busca_maquinas_usuario_idusuario(Query(IdInput{
        id: "idinválido".to_string()
    })).await.is_err());
    
    assert!(deleta_usuario(idusuario).await.is_ok());
    assert!(deleta_maquina_id(id).await.is_ok());
}

// #[tokio::test]

// async fn test_lista_todas_maquinas_err(){
//     // Esse teste falha quando temos máquinas cadastradas no banco :)
//     assert!(lista_todas_maquinas().await.is_err());
// }