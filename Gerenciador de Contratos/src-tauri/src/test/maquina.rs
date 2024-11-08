
#[cfg(test)]

use axum::Json;
use mysql_async::{params, prelude::Queryable};

use crate::{controller::{cria_pool, maquina::{busca_maquina_numserie, cadastra_maquina, estrutura_maquina, EstruturaMaquinaInput}}, model::{erro::MeuErro, maquina::Maquina}};

pub async fn cria_maquina_teste(nomemaquina: &str, numserie: &str, valoraluguel: &str) -> Result<Maquina, String>{
    use axum::Json;
    let maquina = EstruturaMaquinaInput{
        nomemaquina: nomemaquina.to_string(),
        numserie: numserie.to_string(),
        valoraluguel: valoraluguel.to_string()
    };
    let maquina = Json(maquina);
    let maquina = estrutura_maquina(maquina).await.unwrap();
    let maquina_retorno = Maquina{
        idmaquina: maquina.idmaquina.to_string(),
        nomemaquina: maquina.nomemaquina.to_string(),
        numserie: maquina.numserie.to_string(),
        valoraluguel: maquina.valoraluguel,
        disponibilidade: maquina.disponibilidade,
        maquinastatus: maquina.maquinastatus
    };
    match cadastra_maquina(maquina).await{
        Ok(_) => {
            return Ok(maquina_retorno)
        },
        Err(e) => {
            return Err(e.1.0);
        }
    }
}

#[tokio::test]
pub async fn test_cadastra_deleta_maquina(){
    let nomemaquina = "Maquina de Corte";
    let numserie = "MQC123TEST1";
    let valoraluguel = "R$ 4000,00";
    
    assert!(cria_maquina_teste(nomemaquina, numserie, valoraluguel).await.is_ok());

    assert!(_limpa_maquina(numserie).await.is_ok());
}

pub async fn _limpa_maquina(numserie: &str) -> Result<(), String>{
    if numserie.is_empty(){
        return Err(MeuErro::CamposVazios.to_string())
    }
    match deleta_maquina(numserie.to_string()).await{
        Ok(_) => {
            return Ok(())
        },
        Err(e) => {
            println!("{:?}", e);
            return Err(e.to_string())
        }
    }
}

pub async fn deleta_maquina(numserie: String) -> Result <(), mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let resultado = conn.exec_drop("DELETE FROM maquina WHERE numserie = :numserie;", 
    params! {"numserie" => numserie}).await;
    match resultado{
        Ok(_) => {return Ok(())},
        Err(e) => {return Err(mysql_async::Error::Other(Box::new(e)))}
    }
}