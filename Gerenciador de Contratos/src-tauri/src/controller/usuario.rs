use crate::model;
use crate::controller::valida_email;

#[tauri::command]
pub async fn atualiza_email(email: &str) -> Result<(), String>{
    let email = email.trim();
    let pool = model::create_pool().await.map_err(|e| format!("{}", e)).unwrap();
    let resultado_busca = model::busca_email(&pool, email).await;
    match resultado_busca{
        Ok(o) => {
            if o.is_empty() || !valida_email(&o) || o == ""{
                return Ok(())
            }
        },
        Err(_e) => {
            println!("{:?}", _e);
            return Err("Erro ao atualizar o email".to_string());
        }
    }
 
    let r = model::usuario::atualiza_email(&pool, email, email).await;
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