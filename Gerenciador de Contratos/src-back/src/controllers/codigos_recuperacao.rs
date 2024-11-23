use axum::{http::StatusCode, Json};
use serde::{Serialize, Deserialize};

use super::{envia_email::envia_email_codigo, usuarios::{busca_usuario_email, valida_email, EmailInput}};

#[derive(Serialize, Deserialize)]
pub struct CodigoRecuperacaoInput{
    pub codigodigitado: String
}

pub async fn verifica_codigo_recuperacao(input: Json<CodigoRecuperacaoInput>, codigoteste: Json<String>)
    -> Result<(), (StatusCode, Json<String>)>{
    let codigodigitado = input.codigodigitado.to_string();
    if codigodigitado.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("O código não pode estar vazio.".to_string())))
    }

    // buscar um código de verificação no banco

    // exemplo
    let codigoteste = codigoteste.0.to_string();
    let exemplos = ["1234", "0000", "1111", "3333", "0169"];
    let mut exemplos = exemplos.to_vec();
    exemplos.push(&codigoteste);
    for codigo in exemplos{
        if codigo == codigodigitado{
            return Ok(())
        }
    }

    return Err((StatusCode::BAD_REQUEST, Json("Código não encontrado.".to_string())))
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
    let id = match busca_usuario_email(input).await{
        Ok(id) => {id.1.0},
        Err(e) => {
            return Err(e)
        }
    };

    let codigo = match envia_email_codigo(email_, "recuperação de senha"){
        Ok(codigo) => {codigo},
        Err(e) => {
            return Err((
                StatusCode::SERVICE_UNAVAILABLE,
                Json(format!("Não foi possível enviar o e-mail: {}", e))
            ))
        }
    };
    // Inserir o código de recuperação no banco

    return Ok((StatusCode::OK, Json(codigo)))
}