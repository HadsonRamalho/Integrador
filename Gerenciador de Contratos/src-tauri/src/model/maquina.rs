use mysql_async::{params, prelude::Queryable};
use mysql_async::prelude::FromRow;
use serde::Serialize;

use crate::controller;
//Criada estrutura para representar a quantidade de maquinas em estoque
#[derive(FromRow)]
pub struct EstoqueMaquina{
    pub nomemaquina: String,
    pub quantidade: u64
}

#[derive(Serialize, FromRow)]
pub struct Maquina {
    pub idmaquina: String,
    pub nomemaquina: String,
    pub numserie: String,
    pub valoraluguel: f32,
    pub disponibilidade: i8,
    pub maquinastatus: i16
}

pub async fn cadastrar_maquina(maquina: Maquina) -> Result<String, mysql_async::Error> {
    let idmaquina = maquina.idmaquina.clone();
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
            VALUES(:idmaquina, :nomemaquina, :numserie, :valoraluguel, :disponibilidade, :maquinastatus);",
            params! {"idmaquina" => maquina.idmaquina, "nomemaquina" => maquina.nomemaquina,
            "numserie" => maquina.numserie, "valoraluguel" => maquina.valoraluguel, "maquinastatus" => maquina.maquinastatus,
            "disponibilidade" => maquina.disponibilidade},
        )
        .await;
    match resultado_insert {
        Ok(_) => {
            println!("Maquina cadastrada");
            return Ok(idmaquina)
        }
        Err(e) => {
            println!("{:?}", e);
            return Err(e);
        }
    }
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

pub async fn gera_estoque_total() -> Result<Vec<EstoqueMaquina>, mysql_async::Error>{
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let estoque: Vec<EstoqueMaquina> = conn.exec_map("SELECT nomemaquina COUNT(*) AS estoque FROM maquina WHERE disponibilidade = 1 AND 
    maquinastatus = 1 GROUP BY nomemaquina", (), |(nomemaquina, quantidade)| EstoqueMaquina{nomemaquina, quantidade}).await?;
    if estoque.is_empty(){
        return Err(mysql_async::Error::Other(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Não há máquinas em estoque"))));
    }
    Ok(estoque)
}

pub async fn gera_estoque_por_nome(nomemaquina: String) -> Result<EstoqueMaquina, mysql_async::Error>{
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let estoque: Option<EstoqueMaquina> = conn.exec_first("SELECT nomemaquina COUNT(*) AS estoque FROM maquina WHERE nomemaquina = :nome AND 
    disponibilidade = 1 AND maquinastatus = 1", params!{"nome" => nomemaquina}).await?;
    match estoque{
        None => {
            //Criando um erro personalizado para a aplicação.
            return Err(mysql_async::Error::Other(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, 
                "Numero de série não encontrado"))));
        },
        Some(estoque) => {
            return Ok(estoque);

        }

    }

}