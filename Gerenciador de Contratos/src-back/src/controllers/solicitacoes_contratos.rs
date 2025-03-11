use axum::{extract::Query, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::models::{self, solicitacoes_contratos::SolicitacaoContrato};

use super::{cria_conn, gera_hash, usuarios::IdInput};

#[derive(Serialize, Deserialize)]
pub struct SolicitacaoContratoInput{
    pub idlocador: String,
    pub idlocatario: String,
    pub idmaquina: String,
    pub medidatempolocacao: String,
    pub origemsolicitacao: String,
    pub valorsolicitacao: f64,
    pub prazolocacao: f64,
}

pub async fn cadastra_solicitacao_contrato(input: Json<SolicitacaoContratoInput>)
    -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    if input.idlocador.trim().is_empty() || input.idlocatario.trim().is_empty() || 
      input.idmaquina.trim().is_empty() || input.origemsolicitacao.trim().is_empty()
      || input.prazolocacao.to_string().trim().is_empty() || input.medidatempolocacao.trim().is_empty(){
      return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }

    let idlocador = input.idlocador.trim().to_string();
    let idlocatario = input.idlocatario.trim().to_string();
    let idmaquina = input.idmaquina.trim().to_string();
    let origemsolicitacao = input.origemsolicitacao.trim().to_string();
    let statussolicitacao = "Aguardando aprovação".to_string();
    let medidatempolocacao = input.medidatempolocacao.to_string();
    let prazolocacao = input.prazolocacao;
    let valorsolicitacao = input.valorsolicitacao;

    let idsolicitacao = gera_hash(&idmaquina);
    let datasolicitacao = chrono::Utc::now().naive_utc();
    let solicitacao = SolicitacaoContrato{
      idsolicitacao,
      idlocador,
      idlocatario,
      idmaquina,
      medidatempolocacao,
      prazolocacao,
      origemsolicitacao,
      valorsolicitacao,
      statussolicitacao,
      datasolicitacao
    };

    if valorsolicitacao < 1.{
      return Err((StatusCode::BAD_REQUEST, Json("O valor do aluguel não é válido.".to_string())))
    }
    
    let conn = &mut cria_conn()?;
    match models::solicitacoes_contratos::cadastra_solicitacao_contrato(conn, solicitacao).await{
      Ok(id) => {
        return Ok((StatusCode::OK, Json(id)))
      },
      Err(e) => {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
      }
    }
}

pub async fn busca_solicitacao_idsolicitacao(Query(id): Query<IdInput>)
    -> Result<(StatusCode, Json<SolicitacaoContrato>), (StatusCode, Json<String>)>{
    if id.id.trim().is_empty(){
      return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }
    let id = id.id.trim().to_string();

    let conn = &mut cria_conn()?;

    match models::solicitacoes_contratos::busca_solicitacao_idsolicitacao(conn, id).await{
      Ok(solicitacao) => {
        return Ok((StatusCode::OK, Json(solicitacao)))
      },
      Err(e) => {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
      }
    }
}

pub async fn busca_solicitacoes_idlocador(Query(id): Query<IdInput>)
    -> Result<(StatusCode, Json<Vec<SolicitacaoContrato>>), (StatusCode, Json<String>)>{
    if id.id.trim().is_empty(){
      return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }
    let id = id.id.trim().to_string();

    let conn = &mut cria_conn()?;

    let solicitacoes = match models::solicitacoes_contratos::busca_solicitacoes_idlocador(conn, id).await{
      Ok(solicitacoes) => {
        solicitacoes
      },
      Err(e) => {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
      }
    };

    if solicitacoes.is_empty(){
      return Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Você não possui solicitações de contratos.".to_string())))
    }

    return Ok((StatusCode::OK, Json(solicitacoes)))
}

#[derive(Serialize, Deserialize)]
pub struct StatusSolicitacaoInput{
    pub id: String,
    pub status: String
}

pub async fn atualiza_status_solicitacao(input: Json<StatusSolicitacaoInput>)
    -> Result<(StatusCode, Json<SolicitacaoContrato>), (StatusCode, Json<String>)>{
    if input.id.trim().is_empty() || input.status.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }

    let conn = &mut cria_conn()?;

    let id = input.id.trim().to_string();
    let novostatus = input.status.to_string();
    match models::solicitacoes_contratos::atualiza_status_solicitacao(conn, id, novostatus).await{
        Ok(solicitacao) => {
            return Ok((StatusCode::OK, Json(solicitacao)))
        },
        Err(e) => {
          return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}