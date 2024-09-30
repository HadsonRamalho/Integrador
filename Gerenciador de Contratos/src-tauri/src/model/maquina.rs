use std::vec;

use mysql_async::{params, prelude::Queryable};
use mysql_async::prelude::FromRow;
use serde::Serialize;

use crate::controller;

use super::erro::MeuErro;
//Criada estrutura para representar a quantidade de maquinas em estoque
#[derive(FromRow, Serialize, PartialEq)]
pub struct EstoqueMaquina{
    pub quantidade: i32,
    pub nomemaquina: Option<String>
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
    let pool = controller::cria_pool().await?;
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
            return Err(mysql_async::Error::Other(Box::new(MeuErro::SalvarMaquina)));
        }
    }
}

pub async fn buscar_maquina_nome(nome: &str) -> Result<Vec<Maquina>, mysql_async::Error>{
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let nome_like = format!("%{}%", nome);
    let resultado_select = conn.exec("SELECT * FROM maquina WHERE nomemaquina LIKE :nome;", 
    params!{"nome" => nome_like}).await;
    match resultado_select{
        Ok(maquinas) => {            
            if maquinas.is_empty(){
                return Ok(vec![]);
            }
            return Ok(maquinas)
        },
        Err(e) => {
            println!("{:?}", e);
            return Err(mysql_async::Error::Other(Box::new(MeuErro::NomeMaquinaNaoEncontrado)));
        }

    }
}

pub async fn busca_maquina_serie(serie: &str) -> Result<Vec<Maquina>, mysql_async::Error>{
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let maquina: Option<Maquina> = conn.exec_first("SELECT * FROM maquina WHERE numserie = :serie", 
    params! {"serie" => serie}).await?;
    let mut maquina_retorno = vec![];
    match maquina{
        None => {
            //Criando um erro personalizado para a aplicação.
            return Err(mysql_async::Error::Other(Box::new(MeuErro::NumeroSerieNaoEncontrado)));
        },
        Some(maquina) => {
            maquina_retorno.push(maquina);
            return Ok(maquina_retorno);
        }
    }
}

pub async fn gera_estoque_total() -> Result<Vec<EstoqueMaquina>, mysql_async::Error>{
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let estoque: Vec<EstoqueMaquina> = conn.exec_map("SELECT nomemaquina, COUNT(*) AS estoque FROM maquina WHERE disponibilidade = 1 AND 
    maquinastatus = 1 GROUP BY nomemaquina;", (), |(nomemaquina, quantidade)| EstoqueMaquina{nomemaquina, quantidade}).await?;
    if estoque.is_empty(){
        return Err(mysql_async::Error::Other(Box::new(MeuErro::SemMaquinasNoEstoque)));
    }
    Ok(estoque)
}

pub async fn gera_estoque_total_alugadas() -> Result<Vec<EstoqueMaquina>, mysql_async::Error>{
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let estoque: Vec<EstoqueMaquina> = conn.exec_map("SELECT nomemaquina, COUNT(*) AS estoque FROM maquina WHERE disponibilidade = 0 GROUP BY nomemaquina;", (), |(nomemaquina, quantidade)| EstoqueMaquina{nomemaquina, quantidade}).await?;
    if estoque.is_empty(){
        return Err(mysql_async::Error::Other(Box::new(MeuErro::SemMaquinasAlugadas)));
    }
    Ok(estoque)
}

pub async fn gera_estoque_por_nome(nomemaquina: String) -> Result<Vec<EstoqueMaquina>, mysql_async::Error>{
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let nome_like = format!("%{}%", nomemaquina);
    let estoque_invalido: EstoqueMaquina = EstoqueMaquina{quantidade: 0, nomemaquina: None};
    let estoque = conn.exec_map(
        "SELECT COUNT(*) AS quantidade, nomemaquina FROM maquina 
        WHERE nomemaquina LIKE :nome AND disponibilidade = 1 AND maquinastatus = 1;",
         params! {"nome" => nome_like}, |(quantidade, nomemaquina )| 
            EstoqueMaquina{quantidade, nomemaquina}).await?;
    if estoque.is_empty() || estoque.contains(&estoque_invalido){
        return Err(mysql_async::Error::Other(Box::new(MeuErro::MaquinaNaoEncontradaNoEstoque)));
    }
    return Ok(estoque)

}