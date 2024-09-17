use mysql_async::{params, prelude::Queryable};
use mysql_async::prelude::FromRow;
use serde::Serialize;

use crate::controller;

#[derive(Serialize, FromRow)]
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

pub async fn buscar_maquina_nome(nome: &str) -> Result<Vec<Maquina>, mysql_async::Error>{
    let pool = match controller::cria_pool().await{
            Ok(pool) => {
                pool
            },
            Err(e) =>{
                println!("{:?}", e);
                return Err(e);
            }
    };
    let mut conn = pool.get_conn().await?;
    let nome_like = format!("%{}%", nome);
    let resultado_select = conn.exec("SELECT * FROM maquina WHERE nomemaquina LIKE :nome;", 
    params!{"nome" => nome_like}).await;
    match resultado_select{
        Ok(maquinas) => {
            println!("Maquinas encontradas");
            return Ok(maquinas);
        },
        Err(e) =>{
            println!("{:?}", e);
            return Err(e);
        }

    }
}

pub async fn busca_maquina_serie(serie: &str) -> Result<Maquina, mysql_async::Error>{
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let maquina: Option<Maquina> = conn.exec_first("SELECT * FROM maquina WHERE numserie = :serie", 
    params! {"serie" => serie}).await?;
    match maquina{
        None => {
            //Criando um erro personalizado para a aplicação.
            return Err(mysql_async::Error::Other(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, 
                "Numero de série não encontrado"))));
        },
        Some(maquina) => {
            return Ok(maquina);

        }

    }

}