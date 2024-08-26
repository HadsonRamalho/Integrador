use mysql_async::{params, prelude::Queryable};

use crate::controller;

pub struct Maquina{
    pub idmaquina: String,
    pub nomemaquina: String,
    pub numserie: String,
    pub valoraluguel: float
}

pub async fn cadastrar_maquina(maquina:Maquina) -> Result<(), mysql_async::Error>{
    let pool = controller::cria_pool().await.unwrap();
    let mut conn = pool.get_conn().await?;
    let resultado_insert = conn.exec_drop("INSERT INTO maquina 
    VALUES(:idmaquina, :nomemaquina, :numserie, :valoraluguel);", params! {"idmaquina" => idmaquina, "nomemaquina" => nomemaquina, 
    "numserie" => numserie, "valoraluguel" => valoraluguel}).await;
     match resultado_insert{
        Ok(_) => {
            println!("Maquina cadastrada")
        }
        Err(e) =>{
            println!(":{?}", e);
            return Err(e);
        }
    }

    return Ok(());

}

