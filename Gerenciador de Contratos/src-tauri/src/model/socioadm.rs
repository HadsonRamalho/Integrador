use mysql_async::prelude::Queryable;
use crate::model::params;
use crate::controller;

pub struct SocioADM{
    pub idsocio: String,
    pub idendereco: String,
    pub cpf: String,
    pub orgaoemissor: String,
    pub estadocivil: String,
    pub nacionalidade: String,
    pub nomesocio: String,
    pub sociostatus: i16
}

pub async fn _cadastra_socio_adm(socioadm: SocioADM) -> Result<(), mysql_async::Error>{
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
<<<<<<< HEAD
         conn.exec_drop("INSERT INTO socioadm (idsocio, idendereco, cpf, orgaoemissor, estadocivil, nacionalidade, nomesocio, sociostatus)
          VALUES (:idsocio, :idendereco, :cpf, :orgaoemissor, :estadocivil, :nacionalidade, :nomesocio, :sociostatus);", 
=======
         conn.exec_drop("INSERT INTO socioadm (idsocio, idendereco, cpf, orgaoemissor, estadocivil, nacionalidade, nome)
          VALUES (:idsocio, :idendereco, :cpf, :orgaoemissor, :estadocivil, :nacionalidade, :nomesocio);", 
>>>>>>> a3f0d820ac7a495a374d9c19f1a8368738c6d6f1
         params! {"idsocio" =>  socioadm.idsocio, "idendereco" => socioadm.idendereco, "cpf" => socioadm.cpf, "orgaoemissor" => socioadm.orgaoemissor,
            "estadocivil" => socioadm.estadocivil, "nacionalidade" => socioadm.nacionalidade, "nomesocio" => socioadm.nomesocio,
            "sociostatus" => socioadm.sociostatus}).await;
    match resultado_insert{
        Ok(_) => {
            println!("SocioADM cadastrado");
        }, 
        Err(e) => {
            println!("{:?}", e);
            return Err(e);
        }
    }
    return Ok(());
}