use axum::Json;

use crate::controllers::maquinas::{cadastra_maquina, deleta_maquina_id, lista_todas_maquinas, MaquinaInput};

pub fn maquina_padrao(numeroteste: &str) -> MaquinaInput{
    let nome = format!("Maquina Teste {}", numeroteste);
    let numeroserie = format!("TEST-NS{}", numeroteste);
    let valoraluguel: f64 = numeroteste.parse().unwrap_or(169.1);
    let disponivelaluguel = "Sim".to_string();
    let status = "Ativo".to_string();
    let descricao =  format!("Descrição N{}", numeroteste);
    let categoria = "Máquina de Teste".to_string();

    MaquinaInput{
        nome,
        numeroserie,
        valoraluguel,
        disponivelaluguel,
        status,
        descricao,
        categoria
    }
}

#[tokio::test]
async fn test_cadastra_maquina_ok(){
    let maquina = maquina_padrao("001");

    let idsmaquina = cadastra_maquina(Json(maquina)).await.unwrap().1;
    let id = idsmaquina.0.idmaquina.to_string();

    assert!(deleta_maquina_id(id).await.is_ok());
}

#[tokio::test]
async fn test_lista_todas_maquinas_ok(){
    let maquina = maquina_padrao("002");

    let idsmaquina = cadastra_maquina(Json(maquina)).await.unwrap().1;
    let id = idsmaquina.0.idmaquina.to_string();

    assert!(lista_todas_maquinas().await.is_ok());

    assert!(deleta_maquina_id(id).await.is_ok());
}

#[tokio::test]

async fn test_lista_todas_maquinas_err(){
    // Esse teste falha quando temos máquinas cadastradas no banco :)
    assert!(lista_todas_maquinas().await.is_err());
}