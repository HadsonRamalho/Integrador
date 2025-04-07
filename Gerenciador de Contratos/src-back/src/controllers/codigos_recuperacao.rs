use axum::{extract::Query, http::StatusCode, Json};
use chrono::Days;
use diesel::{ExpressionMethods, RunQueryDsl};
use rand::Rng;
use serde::{Serialize, Deserialize};
use crate::{controllers::usuarios::{busca_usuario_id, IdInput}, models::{self, codigos_recuperacao::{cadastra_codigo_recuperacao_db, CodigoRecuperacao}}};

use super::{cria_conn, envia_emails::envia_email_codigo, gera_hash, usuarios::{busca_usuario_email, valida_email, EmailInput}};
use models::codigos_recuperacao::verifica_codigo_recuperacao_db;

#[derive(Serialize, Deserialize)]
pub struct CodigoRecuperacaoInput{
    pub idusuario: String,
    pub codigodigitado: String
}

#[derive(Serialize, Deserialize)]
pub struct CodigoRecuperacaoReturn{
    pub idcodigo: String,
    pub codigo: String
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

#[utoipa::path(
    post,
    tag = "Código de Recuperação",
    path = "/envia_codigo_recuperacao",
    description = "Envia um e-mail com o código de recuperação de senha.",
    responses(
        (
            status = 200, 
            description = "Dados válidos. E-mail enviado e registro salvo no banco.",
            body = String
        ),
        (
            status = 500,
            description = "Erro ao enviar o e-mail; Erro ao salvar os dados no banco; Erro ao buscar o usuário."
        ),
        (
            status = 400,
            description = "Algum dos campos inseridos está incorreto."
        ),
    ),
    request_body = EmailInput    
)]
pub async fn envia_codigo_recuperacao(input: Json<EmailInput>)
    -> Result<(StatusCode, Json<CodigoRecuperacaoReturn>), (StatusCode, Json<String>)>{
    let email_clone = input.email.to_string();
    match valida_email(input).await{
        Ok(_) => {},
        Err(e) => {
            return Err(e)
        } 
    }

    let idusuario = busca_usuario_email(Query(EmailInput{email: email_clone.clone()})).await?.1.0;
    let usuario = busca_usuario_id(Query(IdInput{id: idusuario.clone()})).await?.1.0;
    let origemconta = usuario.origemconta;

    if origemconta != "Sistema"{
        return Err((StatusCode::BAD_REQUEST, 
            Json("Não é possível solicitar a recuperação de senha de uma conta criada usando o Google.".to_string())))
    }

    let codigoreturn = gera_codigo_recuperacao(email_clone.clone()).await?.1.0;
    let res = envia_email_codigo(email_clone.clone(), "recuperação de senha", codigoreturn.codigo.clone()).await?;
    println!("E-mail: {} | Código: {}", email_clone, res.1.0);

    return Ok((StatusCode::OK, Json(codigoreturn)))
}

pub async fn deleta_codigo(id: String)
    -> Result<String, String>{
    // Só utilizar em testes
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

fn gera_i32_aleatorio() -> i32{
    let mut rng = rand::thread_rng();
    let random: i32 = rng.gen_range(1000..=9999);
    random
}

pub async fn gera_codigo_recuperacao(email: String)
    -> Result<(StatusCode, Json<CodigoRecuperacaoReturn>), (StatusCode, Json<String>)>{
    let email_ = email.clone();
    let input = Query(EmailInput{email: email_.clone()});
    let idusuario = busca_usuario_email(input).await?.1.0;

    let datacriacao = chrono::Utc::now().naive_utc();
    let dia = Days::new(1);
    let dataexpiracao = chrono::Utc::now().checked_add_days(dia).unwrap().naive_utc();
    let idcodigo = gera_hash(&email_);

    let codigo = gera_i32_aleatorio().to_string();

    let codigorecuperacao = CodigoRecuperacao{
        codigo: codigo.clone(),
        datacriacao,
        dataexpiracao,
        status: "Não utilizado".to_string(),
        idusuario,
        idcodigo: idcodigo.clone(),
    };

    let conn = &mut cria_conn()?;

    match cadastra_codigo_recuperacao_db(conn, codigorecuperacao).await{
        Ok(codigo) => {
            return Ok((StatusCode::OK, Json(CodigoRecuperacaoReturn{
                idcodigo,
                codigo
            })))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    };
}