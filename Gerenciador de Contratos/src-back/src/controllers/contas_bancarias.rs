use axum::{extract::Query, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::models::{self, contas_bancarias::ContaBancaria};

use super::{cria_conn, gera_hash, usuarios::IdInput};

#[derive(Serialize, Deserialize)]
pub struct ContaBancariaInput{
    pub idusuario: String,
    pub numeroconta: String,
    pub numeroagencia: String,
    pub nomebanco: String
}

pub async fn cadastra_conta_bancaria(input: Json<ContaBancariaInput>)
    -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    if input.idusuario.trim().is_empty() || input.nomebanco.trim().is_empty()
      || input.numeroagencia.trim().is_empty() || input.numeroconta.trim().is_empty(){
      return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }

    let conta: ContaBancaria = {
      let idconta = gera_hash(&input.idusuario);
      let idusuario = input.idusuario.clone();
      let numeroconta = input.numeroconta.clone();
      let numeroagencia = input.numeroagencia.clone();
      let nomebanco = input.nomebanco.clone();

      ContaBancaria{
        idconta,
        idusuario,
        nomebanco,
        numeroagencia,
        numeroconta
      }
    };

    let conn = &mut cria_conn()?;

    match models::contas_bancarias::cadastra_conta_bancaria(conn, conta).await{
      Ok(id) => {
        return Ok((StatusCode::OK, Json(id)))
      },
      Err(e) => {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
      }
    }
}

pub async fn busca_conta_bancaria_idusuario(Query(id): Query<IdInput>)
    -> Result<(StatusCode, Json<ContaBancaria>), (StatusCode, Json<String>)>{
    if id.id.trim().is_empty(){
      return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }

    let id = id.id.trim().to_string();
    let conn = &mut cria_conn()?;

    match models::contas_bancarias::busca_conta_bancaria_idusuario(conn, id).await{
      Ok(conta) => {
        return Ok((StatusCode::OK, Json(conta)))
      },
      Err(e) => {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
      }
    }
}