use mysql_async::prelude::Queryable;
use serde::Serialize;
use crate::model::params;
use crate::controller;
use::mysql_async::prelude::FromRow;
use crate::model::erro::MeuErro;

#[derive(FromRow, Serialize)]
pub struct Locatario{
    pub idlocatario: String,
    pub idendereco: String,
    pub cnpj: String,
    pub nomelocatario: String,
    pub idsocio: String,
    pub locatariostatus: i16
}


pub async fn _cadastra_locatario(locatario: Locatario) -> Result<(), mysql_async::Error>{
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let resultado_insert =
         conn.exec_drop("INSERT INTO locatario (idlocatario, idendereco, cnpj, nomelocatario, idsocio, locatariostatus)
          VALUES (:idlocatario, :idendereco, :cnpj, :nomelocatario, :idsocio, :locatariostatus);", 
         params! {"idlocatario" =>  locatario.idlocatario, "idendereco" => locatario.idendereco, 
         "cnpj" => locatario.cnpj, "nomelocatario" => locatario.nomelocatario, 
         "idsocio" =>locatario.idsocio, "locatariostatus" => locatario.locatariostatus}).await;
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
    let erro_locatario = mysql_async::Error::Other(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound,
        "Erro: Não foi encontrado um locatario com esse nome.")));

    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let nome_like = format!("%{}%", nome);
    let locatario = conn.exec("SELECT * FROM locatario WHERE nomelocatario LIKE :nome", params!{"nome" => nome_like}).await;
    let locatarios;
    match locatario{
        Ok(locatario) =>{
            println!("Locatarios encontrados");
            locatarios = locatario;
        }
        Err(e) => {
            println!("{:?}", e);
            return Err(e);
        }
    }
    if !locatarios.is_empty(){
        return Ok(locatarios)
    }
    return Err(erro_locatario)
}


pub async fn busca_locatario_cnpj(cnpj: &str) -> Result<Vec<Locatario>, mysql_async::Error>{
    let pool = controller::cria_pool().await?;
    let mut conn = 
    match pool.get_conn().await{
        Ok(conn) => {conn},
        Err(e) => {
            let err = mysql_async::Error::Other(Box::new(MeuErro::ConexaoBanco(e)));
            return Err(err)
        }
    };
    let locatario: Option<Locatario> =    
    match conn.exec_first("SELECT * FROM locatario WHERE cnpj = :cnpj", params!{"cnpj" => cnpj}).await {
        Ok(locatario) => {locatario},
        Err(e) => {
            let err = mysql_async::Error::Other(Box::new(MeuErro::ConexaoBanco(e)));
            return Err(err);
        }
    };
    let mut loc: Vec<Locatario> = vec![];
    match locatario{
        None => {
            return Err(mysql_async::Error::Other(Box::new(MeuErro::CnpjNaoEncontrado)))
        },
        Some(locatario) => {
            loc.push(locatario);
            return Ok(loc)
        }
    }
}