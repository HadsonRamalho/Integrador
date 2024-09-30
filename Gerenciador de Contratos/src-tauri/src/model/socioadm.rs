use mysql_async::prelude::{FromRow, Queryable};
use serde::Serialize;
use crate::model::erro::MeuErro;
use crate::model::params;
use crate::controller::{self, cria_pool};

#[derive(FromRow, Serialize)]
pub struct SocioADM{
    pub idsocio: String,
    pub idendereco: String,
    pub nome: String,
    pub cpf: String,
    pub orgaoemissor: String,
    pub estadocivil: String,
    pub nacionalidade: String,
    pub sociostatus: i16,
    pub cnpj: String
}

pub async fn _cadastra_socio_adm(socioadm: SocioADM) -> Result<(), mysql_async::Error>{
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let resultado_insert =
         conn.exec_drop("INSERT INTO socioadm (idsocio, idendereco, cpf, orgaoemissor, estadocivil, nacionalidade,
          nome, sociostatus, cnpj)
          VALUES (:idsocio, :idendereco, :cpf, :orgaoemissor, :estadocivil, :nacionalidade, :nome, :sociostatus, :cnpj);", 
         params! {"idsocio" =>  socioadm.idsocio, "idendereco" => socioadm.idendereco, "cpf" => socioadm.cpf,
            "orgaoemissor" => socioadm.orgaoemissor,
            "estadocivil" => socioadm.estadocivil, "nacionalidade" => socioadm.nacionalidade, "nome" => socioadm.nome,
            "sociostatus" => socioadm.sociostatus, "cnpj" => socioadm.cnpj}).await;
    match resultado_insert{
        Ok(_) => {
            println!("SocioADM cadastrado");
        }, 
        Err(e) => {
            println!("{:?}", e);
            return Err(mysql_async::Error::Other(Box::new(MeuErro::SalvarSocio)));
        }
    }
    return Ok(());
}

pub async fn busca_id_socio_adm(cpf: String) -> Result<String, mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let resultado_busca = 
        conn.exec_first("SELECT idsocio FROM socioadm WHERE cpf = :cpf", params! {"cpf" => cpf}).await;
    let idsocio: Option<String> = match resultado_busca{
        Ok(idsocio) => {idsocio},
        Err(e) => {
            println!("{:?}", e);
            return Err(mysql_async::Error::Other(Box::new(MeuErro::CpfNaoEncontrado)))
        }
    };
    match idsocio{
        Some(idsocio) =>{
            return Ok(idsocio)
        },
        None => {
            return Ok("".to_string())
        }
    }
}

pub async fn busca_socio_adm_id(idsocio: String) -> Result<Vec<SocioADM>, mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let resultado_busca: Result<Vec<SocioADM>, mysql_async::Error> = 
        conn.exec("SELECT * FROM socioadm WHERE idsocio = :idsocio", 
        params! {"idsocio" => idsocio}).await;
    let socio = match resultado_busca{
        Ok(socio) => {
            socio
        },
        Err(e) => {
            println!("{:?}", e);
            return Err(mysql_async::Error::Other(Box::new(MeuErro::SocioNaoEncontrado)))
        }
    };
    if socio.is_empty(){
        return Err(mysql_async::Error::Other(Box::new(MeuErro::SocioNaoEncontrado)))
    }
    return Ok(socio)    
}

pub async fn socio_adm_existente(cpf: &str) -> Result<SocioADM, mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let socioadm: Result<Option<SocioADM>, mysql_async::Error> = 
        conn.exec_first("SELECT * FROM socioadm WHERE cpf = :cpf", params! {"cpf" => cpf} ).await;
    let socioadm = match socioadm{
        Ok(socioadm) => {socioadm},
        Err(e) => {
            println!("{:?}", e);
            return Err(mysql_async::Error::Other(Box::new(MeuErro::SocioNaoEncontrado)))
        }
    };
    match socioadm{
        None => {return Ok(SocioADM {
            idsocio: "".to_string(),
            idendereco: "".to_string(),
            cnpj:"".to_string(),
            cpf: "".to_string(),
            nome: "".to_string(),
            estadocivil: "".to_string(),
            orgaoemissor: "".to_string(),
            nacionalidade: "".to_string(),
            sociostatus: 1,
        })},
        Some(socioadm) => {return Ok(socioadm)}
    }
}