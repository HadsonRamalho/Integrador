use mysql_async::prelude::Queryable;
use crate::model::params;
use crate::controller;

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