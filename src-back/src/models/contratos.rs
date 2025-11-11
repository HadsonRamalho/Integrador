use chrono::NaiveDateTime;
use diesel::{prelude::{Insertable, Queryable}, query_dsl::methods::FilterDsl, ExpressionMethods, PgConnection, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = crate::schema::contratos)]
#[diesel(check_for_backend(diesel::pg::Pg))] 
pub struct Contrato{ 
    pub idcontrato: String,
    pub idlocatario: String,
    pub idlocador: String,
    pub idenderecolocatario: String,
    pub idenderecolocador: String,
    pub idenderecoretirada: String,
    pub idmaquina: String,
    pub idsolicitacaocontrato: String,
    pub idcontabancarialocador: String,
    pub medidatempolocacao: String,
    pub cidadeforo: String,
    pub statuscontrato: String,
    pub prazolocacao: f64,
    pub valorlocacao: f64,
    pub datacontrato: NaiveDateTime,
}

pub async fn cadastra_contrato(conn: &mut PgConnection, contrato: Contrato)
    -> Result<String, String>{
    use crate::schema::contratos::dsl::*;

    let res: Result<Contrato, diesel::result::Error> = diesel::insert_into(contratos)
      .values(contrato)
      .get_result(conn);

    match res{
      Ok(contrato) => {
        return Ok(contrato.idcontrato)
      },
      Err(e) => {
        return Err(e.to_string())
      }
    }
}

pub async fn busca_contrato_id(conn: &mut PgConnection, id: String)
    -> Result<Contrato, String>{
    use crate::schema::contratos::dsl::*;

    let res: Result<Contrato, diesel::result::Error> = contratos.filter(idcontrato.eq(id))
      .get_result(conn);

    match res{
      Ok(contrato) => {
        return Ok(contrato)
      },
      Err(e) => {
        return Err(e.to_string())
      }
    }
}


pub async fn busca_contrato_idsolicitacao(conn: &mut PgConnection, id: String)
    -> Result<Contrato, String>{
    use crate::schema::contratos::dsl::*;

    let res: Result<Contrato, diesel::result::Error> = contratos.filter(idsolicitacaocontrato.eq(id))
      .get_result(conn);

    match res{
      Ok(contrato) => {
        return Ok(contrato)
      },
      Err(e) => {
        return Err(e.to_string())
      }
    }
}