use mysql_async::Pool;

use crate::model::usuario::busca_id_usuario;
use crate::model::{self, usuario};
use crate::controller::valida_email;
use crate::controller;

use super::{gera_hash, verifica_hash};

#[tauri::command]
pub async fn atualiza_email(email: &str) -> Result<(), String>{
    let email: &str = email.trim(); // Utilizar email do usuário atual [Cod. 601]
    let pool: mysql_async::Pool = model::create_pool().await.map_err(|e| format!("{}", e)).unwrap();    
    let resultado_busca: Result<String, mysql_async::Error> = model::busca_email(&pool, "user1@u.com").await;// [Cod. 601]
    match resultado_busca{
        Ok(o) => {
            if o.is_empty() || !valida_email(&o) || o == ""{
                return Err("Email antigo inválido.".to_string()) // [Cod. 601]
            }
        },
        Err(_e) => {
            println!("{:?}", _e);
            return Err("Erro ao atualizar o email".to_string());
        }
    }
 
    let r: Result<(), mysql_async::Error> = model::usuario::atualiza_email(&pool, "user1@u.com", email).await;
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
pub async fn atualiza_senha(email: &str, nova_senha: &str) -> Result<(), String>{
    match valida_senha(nova_senha){
        Ok(_) => {},
        Err(e) => {
            return Err(e)
        }
    }
    let nova_senha = gera_hash(nova_senha.trim());
    let pool = model::create_pool().await.map_err(|e| format!("{}", e)).unwrap();
    let resultado_busca: Result<String, mysql_async::Error> = model::busca_email(&pool, email).await;// [Cod. 601]
    match resultado_busca{
        Ok(o) => {
            if o.is_empty() || !valida_email(&o) || o == ""{ // [Cod. 601] 
                return Ok(())
            }
        },
        Err(_e) => {
            println!("{:?}", _e);
            return Err("Erro ao atualizar a senha".to_string());
        }
    }
    let resultado_atualizacao: Result<(), mysql_async::Error> = model::usuario::atualiza_senha(&pool, email, &nova_senha).await;
    match resultado_atualizacao{
        Ok(()) => {
            return Ok(())
        },
        Err(_e) => {
            println!("Erro ao atualizar a senha");
            return Err("Erro ao atualizar a senha".to_string());
        }
    }
}

#[tauri::command]
pub async fn verifica_token(email: &str, token: &str) -> Result<String, String>{
    let pool = controller::cria_pool().await?;
    let id = busca_id_usuario(&pool, email).await;
    match id{
        Ok(id) =>{
            if id.is_empty(){
                return Err("Erro ao validar o token: Verifique o email.".to_string())
            }
            return Ok(id)
        },
        Err(e) =>{
            return Err(e.to_string())
        }
    }
}

#[tauri::command]
pub async fn busca_id(email: &str) -> Result<String, String>{ //recebe email, retorna ID
    let pool: mysql_async::Pool = controller::cria_pool().await?;
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
    fn contem_numero(s: &str) -> bool {
        s.chars().any(|c| c.is_digit(10))
    }
    if !contem_numero(senha){
        return Err(("Erro: A senha deve conter ao menos um número".to_string()))
    }
    return Ok(()) // Dar Ok após verificar se existe ao menos um número e um caractere especial
}