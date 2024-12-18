use axum::{http::StatusCode, Json};
use chrono::Days;
use diesel::{ExpressionMethods, RunQueryDsl};
use serde::{Serialize, Deserialize};

use crate::models::{self, codigos_recuperacao::{cadastra_codigo_recuperacao_db, CodigoRecuperacao}};

use super::{cria_conn, envia_emails::envia_email_codigo, gera_hash, usuarios::{busca_usuario_email, valida_email, EmailInput}};
use models::codigos_recuperacao::verifica_codigo_recuperacao_db;

#[derive(Serialize, Deserialize)]
pub struct CodigoRecuperacaoInput{
    pub idusuario: String,
    pub codigodigitado: String
}

pub async fn verifica_codigo_recuperacao(input: Json<CodigoRecuperacaoInput>)
    -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    let codigodigitado = input.codigodigitado.trim().to_string();
    if codigodigitado.is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("O código não pode estar vazio.".to_string())))
    }

    let idusuario = input.idusuario.trim().to_string();
    if idusuario.is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("O ID do usuário não pode estar vazio.".to_string())))
    }

    let conn = &mut cria_conn()?;

    match verifica_codigo_recuperacao_db(conn, idusuario, codigodigitado).await{
        Ok(id) => {
            return Ok((StatusCode::OK, Json(id)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}

pub async fn envia_codigo_recuperacao(input: Json<EmailInput>)
    -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    let email_clone = input.email.to_string();
    match valida_email(input).await{
        Ok(_) => {},
        Err(e) => {
            return Err(e)
        }
    }
    let email_ = email_clone.clone();
    let input = Json(EmailInput{email: email_clone});
    let idusuario = busca_usuario_email(input).await?.1.0;

    let codigo = envia_email_codigo(email_, "recuperação de senha").await?.1.0;

    let conn = &mut cria_conn()?;

    let datacriacao = chrono::Utc::now().naive_utc();
    let dia = Days::new(1);
    let dataexpiracao = chrono::Utc::now().checked_add_days(dia).unwrap().naive_utc();
    let idcodigo = gera_hash(&codigo);

    let codigorecuperacao = CodigoRecuperacao{
        codigo,
        datacriacao,
        dataexpiracao,
        status: "Não utilizado".to_string(),
        idusuario,
        idcodigo,
    };

    match cadastra_codigo_recuperacao_db(conn, codigorecuperacao).await{
        Ok(idcodigo) => {
            return Ok((StatusCode::OK, Json(idcodigo)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}

pub async fn deleta_codigo(id: String)
    -> Result<String, String>{
    // usar apenas em testes
    use crate::schema::codigos_recuperacao::dsl::*;

    let conn = &mut cria_conn().unwrap();

    let res: Result<CodigoRecuperacao, diesel::result::Error> = diesel::delete(codigos_recuperacao)
        .filter(idcodigo.eq(id))
        .get_result(conn);

    match res{
        Ok(codigoapagado) => {
            return Ok(codigoapagado.idcodigo)
        },
        Err(e) => {
            return Err(e.to_string())
        }
    }
}