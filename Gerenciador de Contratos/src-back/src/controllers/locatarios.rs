use axum::{extract::Query, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::models::{self, locatarios::Locatario};

use super::{cria_conn, gera_hash, usuarios::{busca_usuario_id, IdInput}};


#[derive(Serialize, Deserialize)]
pub struct LocatarioInput{
  pub idusuario: String,
  pub idendereco: String
}

pub async fn cadastra_locatario(input: Json<LocatarioInput>)
  -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
  if input.idusuario.trim().is_empty() || input.idendereco.trim().is_empty(){
    return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())));
  }

  let res = busca_usuario_id(Query(IdInput{id: input.idusuario.clone()})).await;
  match res{
    Ok(res) => {
      ()
    },
    Err(e) => {
      return Err(e)
    }
  }

  let conn = &mut cria_conn()?;

  let locatario: Locatario = {
    let idlocatario = gera_hash(&input.idusuario);
    let idusuario = input.idusuario.clone();
    let idendereco = input.idendereco.clone();

    Locatario{
      idlocatario,
      idusuario,
      idendereco
    }
  };

  match models::locatarios::cadastra_locatario(conn, locatario).await{
    Ok(id) => {
      return Ok((StatusCode::OK, Json(id)))
    },
    Err(e) => {
      return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
    }
  }
}

pub async fn busca_locatario_idusuario(Query(id): Query<IdInput>)
  -> Result<(StatusCode, Json<Locatario>), (StatusCode, Json<String>)>{
  if id.id.trim().is_empty(){
    return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())));
  }

  let conn = &mut cria_conn()?;
  let idusuario = id.id.trim().to_string();
  
  match models::locatarios::busca_locatario_idusuario(conn, idusuario).await{
    Ok(locatario) => {
      return Ok((StatusCode::OK, Json(locatario)))
    },
    Err(e) => {
      return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
    }
  }
}