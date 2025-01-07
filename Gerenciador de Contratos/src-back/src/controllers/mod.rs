use axum::{http::StatusCode, Json};
use diesel::prelude::*;
use dotenvy::dotenv;
use pwhash::bcrypt;
use std::env;

pub mod codigos_recuperacao;
pub mod envia_emails;
pub mod usuarios;
pub mod maquinas;


pub fn cria_conn() -> Result<PgConnection, (StatusCode, Json<String>)> {
    dotenv().ok();

    let database_url = match env::var("DATABASE_URL"){
        Ok(url) => {
            url
        },
        Err(e) => {
            return Err((StatusCode::SERVICE_UNAVAILABLE, Json(e.to_string())))
        }
    };
    let conn = PgConnection::establish(&database_url);
    match conn{
        Ok(conn) => {
            return Ok(conn)
        },
        Err(e) => {
            return Err((StatusCode::SERVICE_UNAVAILABLE, Json(e.to_string())))
        }
    }
}

pub fn gera_hash(senha: &str) -> String {
    let enc = bcrypt::hash(senha).unwrap();
    return enc;
}

pub fn formata_cnpj(cnpj: &str) -> Result<String, String>{
    let cnpj_numeros: Vec<char> = cnpj
        .chars()
        .filter(|c: &char| c.is_digit(10))
        .collect();
    if cnpj_numeros.len() != 14{
        return Err("Erro: CNPJ de tamanho inválido.".to_string())
    }
    let mut cnpj: Vec<char> = cnpj_numeros;
    cnpj.insert(2, '.');
    cnpj.insert(6, '.');
    cnpj.insert(10, '/');
    cnpj.insert(15, '-');
    let mut cnpjfinal: String = "".to_string();
    for u in cnpj{
        cnpjfinal.push(u);
    }
    return Ok(cnpjfinal);
}

pub fn formata_cpf(cpf: &str) -> Result<String, String>{
    let cpf: Vec<char> = cpf
        .chars()
        .filter(|c: &char| c.is_digit(10))
        .collect();
    if cpf.len() != 11{
        return Err("Erro: CPF de tamanho inválido.".to_string())
    }
    let mut cpf: Vec<char> = cpf;
    cpf.insert(3, '.');
    cpf.insert(7, '.');
    cpf.insert(11, '-');
    let mut cpffinal: String = "".to_string();
    for u in cpf{
        cpffinal.push(u);
    }
    return Ok(cpffinal);
}