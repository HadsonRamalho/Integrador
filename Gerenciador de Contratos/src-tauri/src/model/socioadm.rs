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
            return Err(e);
        }
    }
    return Ok(());
}