use mysql_async::prelude::Queryable;
use serde::Serialize;
use crate::model::params;
use crate::controller;

#[derive(Serialize, Debug)]
pub struct Contrato{
    pub idcontrato: String,
    pub prazolocacao: f32,
    pub dataretirada: String,
    pub valormensal: f32,
    pub vencimento: String,
    pub multaatraso: f32,
    pub jurosatraso: f32,
    pub avisotransferencia: String,
    pub prazodevolucao: String,
    pub cidadeforo: String,
    pub datacontrato: String,
    pub idlocatario: String,
    pub idlocador: String,
    pub idmaquina: String,
    pub enderecoretirada: String
}



pub async fn registra_contrato(contrato: Contrato) -> Result<(), mysql_async::Error>{
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let resultado_insert = conn.exec_drop(
        "INSERT INTO contrato_aluguel (
            idcontrato, prazolocacao, dataretirada, valormensal, vencimento, multaatraso, jurosatraso, avisotransferencia, prazodevolucao, cidadeforo, datacontrato, idlocatario, idlocador, idmaquina, enderecoretirada
        ) VALUES (
            :idcontrato, :prazolocacao, :dataretirada, :valormensal, :vencimento, :multaatraso, :jurosatraso, :avisotransferencia, :prazodevolucao, :cidadeforo, :datacontrato, :idlocatario, :idlocador, :idmaquina, :enderecoretirada
        )",
        params! {
            "idcontrato" => contrato.idcontrato, 
            "prazolocacao" => contrato.prazolocacao, 
            "dataretirada" => contrato.dataretirada, 
            "valormensal" => contrato.valormensal,
            "vencimento" => contrato.vencimento, 
            "multaatraso" => contrato.multaatraso, 
            "jurosatraso" => contrato.jurosatraso, 
            "avisotransferencia" => contrato.avisotransferencia,
            "prazodevolucao" => contrato.prazodevolucao, 
            "cidadeforo" => contrato.cidadeforo, 
            "datacontrato" => contrato.datacontrato,
            "idlocatario" => contrato.idlocatario, 
            "idlocador" => contrato.idlocador, 
            "idmaquina" => contrato.idmaquina, 
            "enderecoretirada" => contrato.enderecoretirada,
        }
    ).await;
    
    match resultado_insert{
        Ok(_) => {
            println!("Contrato cadastrado");
        }, 
        Err(e) => {
            println!("{:?}", e);
            return Err(e);
        }
    }
    return Ok(());


}