use chrono::NaiveDateTime;
use diesel::{prelude::{Insertable, Queryable}, query_dsl::methods::FilterDsl, ExpressionMethods, PgConnection, RunQueryDsl, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Insertable, Serialize, Deserialize, Debug, Clone)]
#[diesel(table_name = crate::schema::solicitacoes_contratos)]
#[diesel(check_for_backend(diesel::pg::Pg))] 
pub struct SolicitacaoContrato{ 
    pub idsolicitacao: String,
    pub idlocador: String, 
    pub idlocatario: String,
    pub idmaquina: String,
    pub medidatempolocacao: String,
    pub origemsolicitacao: String,
    pub statussolicitacao: String,
    pub prazolocacao: f64,
    pub valorsolicitacao: f64,
    pub datasolicitacao: NaiveDateTime 
}

pub async fn cadastra_solicitacao_contrato(conn: &mut PgConnection, solicitacao: SolicitacaoContrato)
    -> Result<String, String>{
    use crate::schema::solicitacoes_contratos::dsl::*;
 
    let res: Result<SolicitacaoContrato, diesel::result::Error> = diesel::insert_into(solicitacoes_contratos)
      .values(solicitacao)
      .get_result(conn);
    
    match res{
      Ok(solicitacao) => {
        return Ok(solicitacao.idsolicitacao)
      },
      Err(e) => {
        return Err(e.to_string())
      }
    }
}

pub async fn busca_solicitacoes_idlocador(conn: &mut PgConnection, id: String)
    -> Result<Vec<SolicitacaoContrato>, String>{
    use crate::schema::solicitacoes_contratos::dsl::*;

    let res: Result<Vec<SolicitacaoContrato>, diesel::result::Error> = solicitacoes_contratos
      .filter(idlocador.eq(id))
      .order_by(datasolicitacao.desc())
      .get_results(conn);
    
    match res{
      Ok(solicitacoes) => {
        return Ok(solicitacoes)
      },
      Err(e) => {
        return Err(e.to_string())
      }
    }
}

pub async fn busca_solicitacoes_idlocatario(conn: &mut PgConnection, id: String)
    -> Result<Vec<SolicitacaoContrato>, String>{
    use crate::schema::solicitacoes_contratos::dsl::*;

    let res: Result<Vec<SolicitacaoContrato>, diesel::result::Error> = solicitacoes_contratos
      .filter(idlocatario.eq(id))
      .order_by(datasolicitacao.desc())
      .get_results(conn);
    
    match res{
      Ok(solicitacoes) => {
        return Ok(solicitacoes)
      },
      Err(e) => {
        return Err(e.to_string())
      }
    }
}

pub async fn busca_solicitacoes_idmaquina(conn: &mut PgConnection, id: String)
    -> Result<Vec<SolicitacaoContrato>, String>{
    use crate::schema::solicitacoes_contratos::dsl::*;

    let res: Result<Vec<SolicitacaoContrato>, diesel::result::Error> = solicitacoes_contratos
      .filter(idmaquina.eq(id))
      .order_by(datasolicitacao.desc())
      .get_results(conn);
    
    match res{
      Ok(solicitacoes) => {
        return Ok(solicitacoes)
      },
      Err(e) => {
        return Err(e.to_string())
      }
    }
}

pub async fn atualiza_status_solicitacao(conn: &mut PgConnection, id: String, novostatus: String)
    -> Result<SolicitacaoContrato, String>{
    use crate::schema::solicitacoes_contratos::dsl::*;

    let res: Result<SolicitacaoContrato, diesel::result::Error> = diesel::update(solicitacoes_contratos)
      .filter(idsolicitacao.eq(id))
      .set(statussolicitacao.eq(novostatus))
      .get_result(conn);

    match res{
      Ok(solicitacao) => {
        return Ok(solicitacao)
      },
      Err(e) => {
        return Err(e.to_string())
      }
    }
}

pub async fn busca_solicitacao_idsolicitacao(conn: &mut PgConnection, id: String)
  -> Result<SolicitacaoContrato, String>{
  use crate::schema::solicitacoes_contratos::dsl::*;

  let res = solicitacoes_contratos
    .filter(idsolicitacao.eq(id))
    .get_result(conn);
  
  match res{
    Ok(solicitacao) => {
      return Ok(solicitacao)
    },
    Err(e) => {
      return Err(e.to_string())
    }
  }
}