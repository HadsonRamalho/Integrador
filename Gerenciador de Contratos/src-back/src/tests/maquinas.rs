use axum::Json;

use crate::controllers::maquinas::{cadastra_maquina, deleta_maquina_id, MaquinaInput};

pub fn maquina_padrao(numeroteste: &str) -> MaquinaInput{
    let nome = format!("Maquina Teste {}", numeroteste);
    let numeroserie = format!("TEST-NS{}", numeroteste);
    let valoraluguel: f64 = numeroteste.parse().unwrap_or(169.1);
    let disponivelaluguel = "Sim".to_string();
    let status = "Ativo".to_string();

    MaquinaInput{
        nome,
        numeroserie,
        valoraluguel,
        disponivelaluguel,
        status
    }
}

#[tokio::test]
async fn test_cadastra_maquina_ok(){
    let maquina = maquina_padrao("001");

    let idsmaquina = cadastra_maquina(Json(maquina)).await.unwrap().1;
    let id = idsmaquina.0.idmaquina.to_string();

    assert!(deleta_maquina_id(id).await.is_ok());
}
