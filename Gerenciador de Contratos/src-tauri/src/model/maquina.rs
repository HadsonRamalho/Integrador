use mysql_async::{params, prelude::Queryable};
use serde::Serialize;

use crate::controller;

#[derive(Serialize)]
pub struct Maquina {
    pub idmaquina: String,
    pub nomemaquina: String,
    pub numserie: String,
    pub valoraluguel: f32,
}

pub async fn cadastrar_maquina(maquina: Maquina) -> Result<(), mysql_async::Error> {
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e)
        }
    };
    let mut conn = pool.get_conn().await?;
    let resultado_insert = conn
        .exec_drop(
            "INSERT INTO maquina 
    VALUES(:idmaquina, :nomemaquina, :numserie, :valoraluguel);",
            params! {"idmaquina" => maquina.idmaquina, "nomemaquina" => maquina.nomemaquina,
            "numserie" => maquina.numserie, "valoraluguel" => maquina.valoraluguel},
        )
        .await;
    match resultado_insert {
        Ok(_) => {
            println!("Maquina cadastrada")
        }
        Err(e) => {
            println!("{:?}", e);
            return Err(e);
        }
    }

    return Ok(());
}
