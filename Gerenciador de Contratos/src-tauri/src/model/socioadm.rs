use std::result;

use mysql_async::prelude::Queryable;
use crate::model::erro::MeuErro;
use crate::model::params;
use crate::controller::{self, cria_pool};

pub struct SocioADM{
    pub idsocio: String,
    pub idendereco: String,
    pub cpf: String,
    pub orgaoemissor: String,
    pub estadocivil: String,
    pub nacionalidade: String,
    pub nome: String,
    pub sociostatus: i16
}

pub async fn _cadastra_socio_adm(socioadm: SocioADM) -> Result<(), mysql_async::Error>{
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let resultado_insert =
         conn.exec_drop("INSERT INTO socioadm (idsocio, idendereco, cpf, orgaoemissor, estadocivil, nacionalidade,
          nome, sociostatus)
          VALUES (:idsocio, :idendereco, :cpf, :orgaoemissor, :estadocivil, :nacionalidade, :nome, :sociostatus);", 
         params! {"idsocio" =>  socioadm.idsocio, "idendereco" => socioadm.idendereco, "cpf" => socioadm.cpf,
            "orgaoemissor" => socioadm.orgaoemissor,
            "estadocivil" => socioadm.estadocivil, "nacionalidade" => socioadm.nacionalidade, "nome" => socioadm.nome,
            "sociostatus" => socioadm.sociostatus}).await;
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
            return Err(mysql_async::Error::Other(Box::new(MeuErro::SocioNaoEncontrado)))
        }
    }
}