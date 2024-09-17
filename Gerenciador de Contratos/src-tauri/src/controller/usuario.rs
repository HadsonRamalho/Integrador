use mysql_async::prelude::Queryable;
use mysql_async::{params, Pool};

use crate::model::usuario::busca_id_usuario;
use crate::model::{self, usuario};
use crate::controller::valida_email;
use crate::controller;
use super::{gera_hash, verifica_hash};

#[tauri::command]
pub async fn atualiza_email(email_antigo: String, email: String) -> Result<(), String>{
    let email: &str = email.trim();
    if !valida_email(email){
        return Err("Erro: Novo email inválido".to_string())
    }
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e.to_string())
        }
    };    
    let resultado_busca: Result<String, mysql_async::Error> = model::busca_email(&pool, &email_antigo).await;
    match resultado_busca{
        Ok(o) => {
            if o.is_empty() || !valida_email(&o) || o == ""{
                return Err("Email antigo inválido.".to_string()) 
            }
        },
        Err(_e) => {
            println!("{:?}", _e);
            return Err("Erro ao atualizar o email".to_string());
        }
    }
 
    let r: Result<(), mysql_async::Error> = model::usuario::atualiza_email(&pool, &email_antigo, email).await;
    match r{
        Ok(()) => {
            return Ok(())
        },
        Err(_e) => {
            println!("Erro ao atualizar o email");
            return Err("Erro ao atualizar o email".to_string());
        }
    }
}

#[tauri::command]
pub async fn atualiza_senha(email: &str, nova_senha: &str) -> Result<String, String>{
    match valida_senha(nova_senha){
        Ok(_) => {},
        Err(e) => {
            return Err(e)
        }
    }
    let nova_senha = gera_hash(nova_senha.trim());
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e.to_string())
        }
    };
    let resultado_busca: Result<String, mysql_async::Error> = model::busca_email(&pool, email).await;
    match resultado_busca{
        Ok(email) => {
            if email.is_empty() || !valida_email(&email) || email == ""{
                return Err("Erro: Email inválido".to_string())
            }
        },
        Err(e) => {
            return Err(e.to_string());
        }
    }
    let resultado_atualizacao: Result<(), mysql_async::Error> = model::usuario::atualiza_senha(&pool, email, &nova_senha).await;
    match resultado_atualizacao{
        Ok(()) => {
            return Ok("Senha atualizada com sucesso!".to_string())
        },
        Err(_e) => {
            println!("Erro ao atualizar a senha");
            return Err("Erro ao atualizar a senha".to_string());
        }
    }
}

#[tauri::command]
pub async fn verifica_token(email: &str, token: &str) -> Result<bool, String>{
    let email = email.trim();
    let token = token.trim();
    if !valida_email(email){
        return Err("Erro ao validar o token: E-mail vazio.".to_string());
    }
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e.to_string())
        }
    };
    let id = busca_id_usuario(&pool, email).await;
    let uid;
    match id{
        Ok(id) =>{
            if id.is_empty(){
                return Err("Erro ao validar o token: Verifique o email.".to_string())
            }
            uid = id;
        },
        Err(e) =>{
            return Err(e.to_string())
        }
    }
    
    let email = _busca_email_usuario(&pool, token).await;
    match email{
        Ok(_) =>{
            if verifica_hash(&email.unwrap(), uid){
                return Ok(true);
            }
            return Err("Token inválido".to_string());
        },
        Err(e) => {
            return Err(e.to_string())            
        }
    }
}

#[tauri::command]
pub async fn busca_id(email: &str) -> Result<String, String>{
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e.to_string())
        }
    };
    let resultado_busca = usuario::busca_id_usuario(&pool, email).await;
    match resultado_busca{
        Ok(id) =>{
            if id.is_empty(){
                return Err("Erro: ID não encontrado. Verifique o e-mail.".to_string());
            }
            return Ok(id);
        },
        Err(e) =>{
            return Err(e.to_string());
        }
    }
}

pub fn valida_senha(senha: &str) -> Result<(), String>{
    if senha.len() < 8{
        return Err("Erro: A senha é muito curta".to_string());
    }
    if senha.is_empty() || senha == ""{
        return Err("Erro: A senha não pode estar vazia".to_string())
    }
    if !senha.chars().any(|c| c.is_ascii_digit()){
        return Err("Erro: A senha deve conter ao menos um número".to_string())
    }
    if !senha.chars().any(|c| c.is_ascii_punctuation()){
        return Err("Erro: A senha deve conter ao menos um símbolo".to_string())
    }
    return Ok(())
}

#[tauri::command]
pub async fn busca_email_usuario(id: String) -> Result<String, String>{
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e.to_string())
        }
    };
    let email = _busca_email_usuario(&pool, &id).await;
    match email{
        Ok(_) => { return Ok(email.unwrap());
    }, Err(e) => {
        return Err(e.to_string());
    }
    }
}

pub async fn _busca_email_usuario(pool: &Pool, id: &str) -> Result<String, mysql_async::Error>{
    let mut conn = pool.get_conn().await?;
    let email_usuario: Option<String> = conn.exec_first("SELECT email FROM usuarios WHERE UUID = :id;", 
    params!{"id" => id}).await?;
    let server_error = mysql_async::ServerError{
        code: 1045, //Código de erro
        message: "ID inválido.".to_string(),
        state: "28000".to_string()
    };
    match email_usuario{
        None => {
            return Err(mysql_async::Error::Server(server_error));
        },
        Some(_) => {
            return Ok(email_usuario.unwrap());
        }
    }
}

#[tauri::command]
pub async fn busca_nome_usuario(id: String) -> Result<String, String>{
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e.to_string())
        }
    };
    let nome = _busca_nome_usuario(&pool, &id).await;
    match nome{
        Ok(_) => { return Ok(nome.unwrap());
    }, Err(e) => {
        return Err(e.to_string());
    }
    }
}

pub async fn _busca_nome_usuario(pool: &Pool, id: &str) -> Result<String, mysql_async::Error>{
    let mut conn = pool.get_conn().await?;
    let nome_usuario: Option<String> = conn.exec_first("SELECT nome_completo FROM usuarios WHERE UUID = :id;", 
    params!{"id" => id}).await?;
    let server_error = mysql_async::ServerError{
        code: 1045, 
        message: "ID inválido.".to_string(),
        state: "28000".to_string()
    };
    match nome_usuario{
        None => {
            return Err(mysql_async::Error::Server(server_error));
        },
        Some(_) => {
            return Ok(nome_usuario.unwrap());
        }
    }
}

#[tauri::command]
pub async fn busca_cnpj_usuario(id: String) -> Result<String, String>{
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e.to_string())
        }
    };
    let cnpj = _busca_cnpj_usuario(&pool, &id).await;
    match cnpj{
        Ok(_) => { 
            return Ok(cnpj.unwrap());
        }, 
        Err(e) => {
            return Err(e.to_string());
        }
    }
}

pub async fn _busca_cnpj_usuario(pool: &Pool, id: &str) -> Result<String, mysql_async::Error>{
    let mut conn = pool.get_conn().await?;
    let cnpj: Option<String> = conn.exec_first("SELECT cnpj FROM usuarios WHERE UUID = :id;", 
    params!{"id" => id}).await?;
    let server_error = mysql_async::ServerError{
        code: 1045, 
        message: "ID inválido.".to_string(),
        state: "28000".to_string()
    };
    println!("{:?}", cnpj);
    match cnpj{
        None => {
            return Err(mysql_async::Error::Server(server_error));
        },
        Some(cnpj) => {
            if cnpj.is_empty(){
                return Err(mysql_async::Error::Server(server_error));
            }
            return Ok(cnpj);
        }
    }
}