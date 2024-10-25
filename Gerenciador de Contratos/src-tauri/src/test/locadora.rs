use axum::Json;
use mysql_async::{params, prelude::Queryable};

use crate::{controller::{cria_pool, locadora::{cadastra_locadora, estrutura_locadora, LocadoraInput}}, model::locadora::Locadora};

pub async fn cria_locadora_teste(cnpj: &str, idendereco: &str, numerocontabanco: &str, 
    numeroagenciabanco: &str, nomebanco: &str, nomelocadora: &str,
    idsocio: &str) -> Result<Json<Locadora>, String>{
    use axum::Json;
    let cnpj = cnpj.to_string();

    let locadora = LocadoraInput{
        idendereco: idendereco.to_string(),
        cnpj: cnpj.to_string(),
        numerocontabanco: numerocontabanco.to_string(),
        numeroagenciabanco: numeroagenciabanco.to_string(),
        nomebanco: nomebanco.to_string(),
        nomelocadora: nomelocadora.to_string(),
        idsocio: idsocio.to_string(),
    };

    let locadora = estrutura_locadora(Json(locadora)).unwrap();
    let locadora_retorno = locadora.clone();
    match cadastra_locadora(locadora).await{
        Ok(_) => {
            return Ok(locadora_retorno)
        },
        Err(e) => {
            return Err(e)
        }
    }
}

pub async fn _limpa_locadora(idlocadora: String) -> Result<(), String>{
    match deleta_locadora(idlocadora).await{
        Ok(_) => {
            return Ok(())
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}

async fn deleta_locadora(idlocadora: String) -> Result<(), mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    match conn.exec_drop("DELETE FROM locadora WHERE idlocadora = :idlocadora", 
    params! {"idlocadora" => idlocadora}).await{
        Ok(_) => {
            return Ok(())
        },
        Err(e) =>{
            return Err(e)
        }
    }
}