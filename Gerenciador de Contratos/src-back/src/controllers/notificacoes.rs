use axum::{extract::Query, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::models::{self, notificacoes::Notificacao};

use super::{cria_conn, gera_hash, usuarios::IdInput};

#[derive(Serialize, Deserialize)]
pub struct NotificacaoInput{
    pub idusuario: String,
    pub titulo: String,
    pub mensagem: String,
    pub onclick: String,
}

pub async fn cadastra_notificacao(input: Json<NotificacaoInput>)
    -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    if input.idusuario.trim().is_empty() || input.mensagem.trim().is_empty()
      || input.onclick.is_empty() || input.titulo.trim().is_empty(){
      return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }
  
    let idnotificacao = gera_hash(&input.idusuario);
    let datacriacao = chrono::Utc::now().naive_utc();

    let notificacao = Notificacao{
      idnotificacao,
      idusuario: input.idusuario.to_string(),
      titulo: input.titulo.to_string(),
      mensagem: input.mensagem.to_string(),
      onclick: input.onclick.to_string(),
      status: "Não lida".to_string(),
      datacriacao
    };

    let conn = &mut cria_conn()?;

    match models::notificacoes::cadastra_notificacao(conn, notificacao).await{
      Ok(notificacao) => {
        return Ok((StatusCode::OK, Json(notificacao.idnotificacao)))
      },
      Err(e) => {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
      }
    }
}

pub async fn busca_notificacoes_idusuario(Query(id): Query<IdInput>)
    -> Result<(StatusCode, Json<Vec<Notificacao>>), (StatusCode, Json<String>)>{
    if id.id.trim().is_empty(){
      return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }

    let id = id.id.trim().to_string();

    let conn = &mut cria_conn()?;

    match models::notificacoes::busca_notificacoes_idusuario(conn, id).await{
      Ok(res) => {
        if res.is_empty(){
          return Err((StatusCode::INTERNAL_SERVER_ERROR, Json("Esse usuário não possui notificações.".to_string())))
        }
        return Ok((StatusCode::OK, Json(res)))
      },
      Err(e) => {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
      }
    }
}