use mysql_async::prelude::Queryable;
use crate::model::params;
use crate::controller;

pub struct Contrato{
    pub idcontrato: String,
    pub prazolocacao: Float,
    pub dataretirada: Date,
    pub valormensal: Float,
    pub vencimento: Date,
    pub multaatraso: Float,
    pub jurosatraso: Float,
    pub avisotransferencia: String,
    pub prazodevolucao: Date,
    pub cidadeforo: String,
    pub datacontrato: Date,
    pub idlocatario: String,
    pub idlocador: String,
    pub idaluguelmaquina: String,
    pub idenderecoretirada: String
}

pub async fn registra_contrato(contrato: Contrato) -> Result<(), mysql_async::Error>{
    let pool = controller::cria_pool().await.unwrap();
    let mut conn = pool.get_conn().await?;
    let resultado_insert =
         conn.exec_drop("INSERT INTO contrato_aluguel (idcontrato, prazolocacao, dataretirada, 
        valormensal, vencimento, multaatraso, jurosatraso, avisotransferencia, prazodevolucao, cidadeforo, datacontrato, idlocatario,
        idlocador, idaluguelmaquina, idenderecoretirada);
        VALUES (:idcontrato, :prazolocacao, :dataretirada, :valormensal,
        :multaatraso, :jurosatraso, :avisotransferencia, :prazodevolucao, :cidadeforo, :datacontrato, :idlocatario,
        :idlocador, :idaluguelmaquina, :idenderecoretirada);", 
         params! {"idcontrato" =>  contrato.idcontrato, "prazolocacao" => contrato.prazolocacao, "dataretirada" => contrato.dataretirada, 
        "valormensal" => contrato.valormensal,
        "vencimento" => contrato.vencimento, "multaattraso" => contrato.multaatraso, 
        "jurosatraso" => contrato.jurosatraso, "avisotransferencia" => contrato.avisotransferencia,
        "prazodevolucao" => contrato.prazodevolucao, "cidadeforo" => contrato.cidadeforo, "datacontrato" => contrato.datacontrato,
        "idlocatario" => contrato.idlocatario, "idlocador" => contrato.idlocador, "idaluguelmaquina" => contrato.idaluguelmaquina,
        "idenderecoretirada" => contrato.idenderecoretirada}).await;
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