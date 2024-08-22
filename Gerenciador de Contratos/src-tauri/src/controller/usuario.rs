use crate::model;
use crate::controller::valida_email;

use super::{dec_senha, enc_senha};

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
    let nova_senha = enc_senha(nova_senha.trim());
    let pool = model::create_pool().await.map_err(|e| format!("{}", e)).unwrap();
    let resultado_busca: Result<String, mysql_async::Error> = model::busca_email(&pool, email).await;// [Cod. 601]
    if nova_senha.is_empty() || nova_senha == ""{
        return Err("Erro: sua nova senha não pode estar vazia.".to_string());
    }
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
pub async fn verifica_token(email: &str, token: &str) -> Result<(), ()>{
    match dec_senha(email, token.to_string()){
        true =>{
            println!("Token verificado");
            return Ok(())
        },
        false => {
            println!("Falha na verificação do token");
            return Err(())
        }
    }
}

#[tauri::command]
pub async fn busca_id() -> Result<String, String>{
    return Ok("$2b$10$nEmaaQ8g53SKbGmdF7vltej675xjgCKN0tMBWYpaWj8KxZWrUkoFi".to_string());
}

#[tauri::command]
pub async fn verifica_senha(){

}

pub fn valida_senha(senha: &str) -> bool{
    if senha.len() < 8 || senha.is_empty()
    || senha == ""{
        return false;
    }
    return true;
}