use axum::Json;
use chrono::NaiveDateTime;
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::models::{self, solicitacoes_contratos::SolicitacaoContrato};

use super::{cria_conn, gera_hash};

#[derive(Serialize, Deserialize)]
pub struct SolicitacaoContratoInput{
    pub idlocador: String,
    pub idlocatario: String,
    pub idmaquina: String,
    pub prazolocacao: f64,
    pub medidatempolocacao: String,
    pub origemsolicitacao: String,
    pub statussolicitacao: String,
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
      statussolicitacao,
      datasolicitacao
    };
    
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