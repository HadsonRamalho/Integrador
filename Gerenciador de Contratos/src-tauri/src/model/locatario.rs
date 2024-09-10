use mysql_async::prelude::Queryable;
use serde::Serialize;
use crate::model::params;
use crate::controller;
use::mysql_async::prelude::FromRow;

#[derive(FromRow, Serialize)]
pub struct Locatario{
    pub idlocatario: String,
    pub idendereco: String,
    pub cnpj: String,
    pub nomelocatario: String,
    pub idsocio: String
}

pub async fn _cadastra_locatario(locatario: Locatario) -> Result<(), mysql_async::Error>{
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e)
        }
    };
    let mut conn = pool.get_conn().await?;
    let resultado_insert =
         conn.exec_drop("INSERT INTO locatario (idlocatario, idendereco, cnpj, nomelocatario, idsocio)
          VALUES (:idlocatario, :idendereco, :cnpj, :nomelocatario, :idsocio);", 
         params! {"idlocatario" =>  locatario.idlocatario, "idendereco" => locatario.idendereco, 
         "cnpj" => locatario.cnpj, "nomelocatario" => locatario.nomelocatario, 
         "idsocio" =>locatario.idsocio}).await;
    match resultado_insert{
        Ok(_) => {
            println!("Locatario cadastrado");
        }, 
        Err(e) => {
            println!("{:?}", e);
            return Err(e);
        }
    }
    return Ok(());
}

pub async fn busca_locatario_nome(nome: &str) -> Result<Vec<Locatario>, mysql_async::Error>{
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let nome_like = format!("%{}%", nome);
    let locatario = conn.exec("SELECT * FROM locatario WHERE nomelocatario LIKE :nome", params!{"nome" => nome_like}).await;
    match locatario{
        Ok(locatario) =>{
            println!("Locatarios encontrados");
            return Ok(locatario);
        }
        Err(e) => {
            println!("{:?}", e);
            return Err(e);
        }
    }
}

pub async fn busca_locatario_cnpj(cnpj: &str) -> Result<Locatario, mysql_async::Error>{
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let locatario: Option<Locatario> = conn.exec_first("SELECT * FROM locatario WHERE cnpj = :cnpj", params!{"cnpj" => cnpj}).await?;
    match locatario {
        None => {
            return Err(mysql_async::Error::Other(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound,
                "CNPJ não encontrado"))));
        }
        Some(locatario) => {
            return Ok(locatario);
        }
    }
}