use mysql_async::prelude::{FromRow, Queryable};
use mysql_async::Row;
use serde::Serialize;
use crate::controller::contrato::cria_vetor_contratos;
use crate::model::erro::MeuErro;
use crate::model::params;
use crate::controller::{self, cria_pool};

#[derive(Serialize, Debug, FromRow)]
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
            return Err(mysql_async::Error::Other(Box::new(MeuErro::SalvarContrato)));
        }
    }
    return Ok(());


}

pub async fn busca_contrato_nome_maquina(nome_maquina: String, cnpj: String) -> Result<Vec<Contrato>, mysql_async::Error> {
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e)
        }
    };
    let mut conn = pool.get_conn().await?;
    let cnpj = cnpj.trim();
    let nome_like = format!("%{}%", nome_maquina);

    let rows: Vec<Row> = conn.exec(
        "SELECT ca.idcontrato, ca.prazolocacao, ca.dataretirada, ca.valormensal, ca.vencimento,
       ca.multaatraso, ca.jurosatraso, ca.avisotransferencia, ca.prazodevolucao, 
       ca.cidadeforo, ca.datacontrato, ca.idlocatario, ca.idlocador, ca.idmaquina, 
       ca.enderecoretirada
        FROM contrato_aluguel ca
        JOIN locadora ld ON ca.idlocador = ld.idlocadora
        JOIN maquina ma ON ca.idmaquina = ma.idmaquina
        WHERE ma.nomemaquina LIKE :nome_maquina AND ld.cnpj = :cnpj
        ORDER BY ca.valormensal DESC;",
        params! { "nome_maquina" => nome_like, "cnpj" => cnpj }
    ).await?;

    let contratos: Vec<Contrato> = controller::contrato::cria_vetor_contratos(rows);

    Ok(contratos)
}

pub async fn busca_contratos_a_vencer(cnpj: String) -> Result<Vec<Contrato>, mysql_async::Error> {
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e)
        }
    };
    let mut conn = pool.get_conn().await?;
    let cnpj = cnpj.trim();


    let rows: Vec<Row> = conn.exec(
        "SELECT ca.idcontrato, ca.prazolocacao, ca.dataretirada, ca.valormensal, ca.vencimento,
       ca.multaatraso, ca.jurosatraso, ca.avisotransferencia, ca.prazodevolucao, 
       ca.cidadeforo, ca.datacontrato, ca.idlocatario, ca.idlocador, ca.idmaquina, 
       ca.enderecoretirada
        FROM contrato_aluguel ca
        JOIN locadora ld ON ca.idlocador = ld.idlocadora
        JOIN maquina ma ON ca.idmaquina = ma.idmaquina
        WHERE ld.cnpj = :cnpj
        AND ca.vencimento > CURDATE()
        ORDER BY ca.valormensal DESC;",
        params! {"cnpj" => cnpj }
    ).await?;

    let contratos = controller::contrato::cria_vetor_contratos(rows);
    println!("{:?}", contratos);

    Ok(contratos)
}

pub async fn busca_contrato_numserie_maquina(numserie: String, cnpj: String) -> Result<Vec<Contrato>, mysql_async::Error>{
    let pool = controller::cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let contrato: Vec<Row> = conn.exec("SELECT * FROM contrato_aluguel
        JOIN maquina ON contrato_aluguel.idmaquina = maquina.idmaquina
        JOIN locadora ON contrato_aluguel.idlocador = locadora.idlocadora
        WHERE maquina.numserie = :numserie
            AND locadora.cnpj = :cnpj;",
         params!{"numserie" => numserie, "cnpj" => cnpj}, ).await?;
    if contrato.is_empty(){
        return Err(mysql_async::Error::Other(Box::new(MeuErro::ContratoNaoEncontrado)))
    }
    let contrato = controller::contrato::cria_vetor_contratos(contrato);
    return Ok(contrato)
}

pub async fn busca_contrato_nome_locatario(nomelocatario: String, cnpj: String) -> Result<Vec<Contrato>, mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let nome_like = format!("%{}%", nomelocatario); 
    let resultado_busca: Vec<Row> = 
        conn.exec("SELECT * FROM contrato_aluguel
        JOIN locatario ON contrato_aluguel.idlocatario = locatario.idlocatario
        JOIN locadora ON contrato_aluguel.idlocador = locadora.idlocadora
        WHERE locatario.nomelocatario LIKE :nomelocatario
            AND locadora.cnpj = :cnpj;", 
        params! {"nomelocatario" => nome_like, "cnpj" => cnpj}).await?;
    if resultado_busca.is_empty(){
        return Err(mysql_async::Error::Other(Box::new(MeuErro::ContratoNaoEncontrado)))
    }
    let contratos = cria_vetor_contratos(resultado_busca);
    return Ok(contratos);
}

pub async fn busca_contrato_cnpj_locatario(cnpjlocatario: String, cnpj: String) -> Result<Vec<Contrato>, mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let resultado_busca: Vec<Row> = 
        conn.exec("SELECT * FROM contrato_aluguel
        JOIN locatario ON contrato_aluguel.idlocatario = locatario.idlocatario
        JOIN locadora ON contrato_aluguel.idlocador = locadora.idlocadora
        WHERE locatario.cnpj = :cnpjlocatario
            AND locadora.cnpj = :cnpj;", 
        params! {"cnpjlocatario" => cnpjlocatario, "cnpj" => cnpj}).await?;
    if resultado_busca.is_empty(){
        return Err(mysql_async::Error::Other(Box::new(MeuErro::ContratoNaoEncontrado)))
    }
    let contratos = cria_vetor_contratos(resultado_busca);
    return Ok(contratos);
}