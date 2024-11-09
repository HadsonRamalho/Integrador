use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{models::{self, usuarios::{Usuario}}, schema::usuarios::documento};

use super::{formata_cnpj, formata_cpf, gera_hash};

#[derive(Deserialize, Serialize, Clone)]
pub struct UsuarioInput{
    pub nome: String,
    pub email: String,
    pub senha: String,
    pub documento: String
}

#[derive(Deserialize, Validate)]
pub struct EmailInput{
    #[validate(email)]
    pub email: String
}

pub async fn estrutura_usuario(usuario: Json<UsuarioInput>)
    ->Result<(StatusCode, Json<Usuario>), (StatusCode, Json<String>)>{

    match valida_usuario(usuario.0.clone()).await{
        Ok(_) => {},
        Err(e) => {
            return Err((StatusCode::BAD_REQUEST, Json(e)))
        }
    }


    let email = usuario.email.to_string();
    let nome = usuario.nome.to_string();
    let senha = usuario.senha.to_string();
    let documento_ = usuario.documento.to_string();

    let idusuario = gera_hash(&email);

    let usuario = Usuario{
        nome,
        email,
        senha,
        documento: documento_,
        idusuario
    };
    return  Ok((StatusCode::OK, Json(usuario)));
}

pub async fn cadastra_usuario(usuario: Json<Usuario>)
    -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    match valida_usuario(UsuarioInput{
      email: usuario.email.to_string(),
      nome: usuario.nome.to_string(),
      senha: usuario.senha.to_string(),
      documento: usuario.documento.to_string()  
    }).await{
        Ok(_) => {},
        Err(e) => {
            println!("{:?}", e);
            return Err((StatusCode::BAD_REQUEST, Json(e)))
        }
    }

    let idusuario = usuario.0.idusuario.clone();
    let usuario = usuario.0;
    match models::usuarios::cadastra_usuario((usuario)).await{
        Ok(_) => {
            return Ok((StatusCode::CREATED, Json(idusuario)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }

}

pub async fn valida_usuario(usuario: UsuarioInput) -> Result<(), String>{
    let nome = usuario.nome.to_string();
    if nome.trim().is_empty(){
        return Err("Erro ao validar o e-mail.".to_string())
    }

    let email = usuario.email.to_string();
    match valida_email(Json(EmailInput{email: email.clone()})).await{
        Ok(_) => {},
        Err(e) => {
            return Err(e.1.0)
        }
    }

    let senha = usuario.senha.to_string();
    let senha = gera_hash(&senha);

    let documento_ = usuario.documento.to_string();
    let documento_ = match formata_documento(&documento_){
        Ok(documento_) => {
            documento_
        },
        Err(e) => {
            return Err(e)
        }
    };
    return Ok(())
}

pub async fn valida_email(input: Json<EmailInput>) -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)> {
    match input.validate(){
         Ok(_) => {
             return Ok((StatusCode::OK, Json(input.0.email)))
         },
         Err(e) => {
             return Err((StatusCode::BAD_REQUEST, Json(e.to_string())))
         }
     }
}

pub fn formata_documento(documento_: &str) -> Result<String, String>{
    match formata_cpf(documento_){
        Ok(cpf) => {
            return Ok(cpf)
        },
        Err(_) => {
        }
    }
    match formata_cnpj(documento_){
        Ok(cnpj) => {
            return Ok(cnpj)
        },
        Err(_) => {
            return Err("Não é um documento válido.".to_string())
        }
    }
}