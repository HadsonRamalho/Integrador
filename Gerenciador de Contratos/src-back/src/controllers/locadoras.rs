use axum::{extract::Query, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::models::{self, locadoras::Locadora};

use super::{cria_conn, gera_hash, usuarios::IdInput};


#[derive(Serialize, Deserialize)]
pub struct LocadoraInput{
  pub idusuario: String,
  pub idendereco: String,
  pub idconta: String
}

pub async fn cadastra_locadora(input: Json<LocadoraInput>)
  -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
  if input.idusuario.trim().is_empty() || input.idendereco.trim().is_empty()
    || input.idconta.trim().is_empty(){
    return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())));
  }

  let conn = &mut cria_conn()?;

  let locadora: Locadora = {
    let idlocadora = gera_hash(&input.idusuario);
    let idusuario = input.idusuario.clone();
    let idendereco = input.idendereco.clone();
    let idconta = input.idconta.clone();

    Locadora{
      idlocadora,
      idusuario,
      idendereco,
      idconta
    }
  };

  match models::locadoras::cadastra_locadora(conn, locadora).await{
    Ok(id) => {
      return Ok((StatusCode::OK, Json(id)))
    },
    Err(e) => {
      return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
    }
  }
}

pub async fn busca_locadora_idusuario(Query(id): Query<IdInput>)
  -> Result<(StatusCode, Json<Locadora>), (StatusCode, Json<String>)>{
  if id.id.trim().is_empty(){
    return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())));
  }

  let conn = &mut cria_conn()?;
  let idusuario = id.id.trim().to_string();
  
  match models::locadoras::busca_locadora_idusuario(conn, idusuario).await{
    Ok(locatario) => {
      return Ok((StatusCode::OK, Json(locatario)))
    },
    Err(e) => {
      return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
    }
  }
}