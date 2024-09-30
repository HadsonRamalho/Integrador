use mysql_async::prelude::{FromRow, Queryable};
use crate::model::erro::MeuErro;
use crate::model::params;
use crate::controller::{self, cria_pool};

#[derive(FromRow)]
pub struct Locadora{
    pub idlocadora: String,
    pub idendereco: String,
    pub cnpj: String,
    pub numerocontabanco: String,
    pub numeroagenciabanco: String,
    pub nomebanco: String,
    pub nomelocadora: String,
    pub idsocio: String,
    pub locadorastatus: i16
}

pub async fn _cadastra_locadora(locadora: Locadora) -> Result<(), mysql_async::Error>{
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let resultado_insert =
         conn.exec_drop("INSERT INTO locadora (idlocadora, idendereco, cnpj, 
         numerocontabanco, numeroagenciabanco, nomebanco, nomelocadora, idsocio, locadorastatus)
          VALUES (:idlocadora, :idendereco, :cnpj, :numerocontabanco, :numeroagenciabanco, :nomebanco, :nomelocadora, :idsocio, :locadorastatus);", 
         params! {"idlocadora" =>  locadora.idlocadora, "idendereco" => locadora.idendereco, "cnpj" => locadora.cnpj, 
            "numerocontabanco" => locadora.numerocontabanco,
            "numeroagenciabanco" => locadora.numeroagenciabanco, "nomebanco" => locadora.nomebanco, 
            "nomelocadora" => locadora.nomelocadora, "idsocio" => locadora.idsocio, "locadorastatus" => locadora.locadorastatus}).await;
    match resultado_insert{
        Ok(_) => {
            println!("Locadora cadastrada");
        }, 
        Err(e) => {
            println!("{:?}", e);
            return Err(mysql_async::Error::Other(Box::new(MeuErro::SalvarLocadora)));
        }
    }
    return Ok(());
}

pub async fn _busca_id_locadora(cnpj: &str) -> Result<String, mysql_async::Error>{

    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e)
        }
    };
    let mut conn = pool.get_conn().await?;
    let resultado_busca: Result<Option<String>, mysql_async::Error> = conn.exec_first("SELECT idlocadora FROM locadora WHERE cnpj = :cnpj",
     params!{"cnpj" => cnpj}).await;
    match resultado_busca{
        Ok(id) => {
            match id {
                Some(id) => {
                    return Ok(id);
                }, None =>{
                    return Ok("".to_string());
                }
            }
        },
        Err(e) => {
            println!("{:?}", e);
            return Err(mysql_async::Error::Other(Box::new(MeuErro::CnpjLocadoraNaoEncontrado)));
        }
    }
}

pub async fn locadora_existente(cnpj: &str) -> Result<Locadora, mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let locadora: Result<Option<Locadora>, mysql_async::Error> = 
        conn.exec_first("SELECT * FROM locadora WHERE cnpj = :cnpj", params! {"cnpj" => cnpj} ).await;
    let locadora = match locadora{
        Ok(locadora) => {locadora},
        Err(e) => {
            println!("{:?}", e);
            return Err(mysql_async::Error::Other(Box::new(MeuErro::CnpjLocadoraNaoEncontrado)))
        }
    };
    match locadora{
        None => {return Ok(Locadora {idlocadora: "".to_string(),
            idendereco: "".to_string(),
            idsocio: "".to_string(),
            cnpj:"".to_string(),
            numerocontabanco: "".to_string(),
            numeroagenciabanco: "".to_string(),
            nomebanco: "".to_string(),
            nomelocadora: "".to_string(),
            locadorastatus: 1,
        })},
        Some(locadora) => {return Ok(locadora)}
    }
}