use crate::model::Pool;
use mysql_async::{params, prelude::Queryable};

pub async fn atualiza_email(pool: &Pool, email_antigo: &str, email_novo: &str) -> Result<(), mysql_async::Error>{
    let mut conn = pool.get_conn().await?;
    let resultado_conexao = conn.exec_drop("UPDATE usuarios SET email = :email_novo WHERE email = :email_antigo;", 
    params!{"email_novo" => email_novo, "email_antigo" => email_antigo}).await;
    match resultado_conexao{
        Ok(()) => {
            return Ok(());
        },
        Err(e ) => {
            return Err(e)

        }

    }
}

pub async fn atualiza_senha(pool: &Pool, email: &str, senha_nova: &str) -> Result<(), mysql_async::Error>{
    let mut conn = pool.get_conn().await?;
    let resultado_conexao = conn.exec_drop("UPDATE usuarios SET senha = :senha_nova WHERE email = :email;", 
    params!{"senha_nova" => senha_nova, "email" => email}).await;
    match resultado_conexao{
        Ok(()) => {
            return Ok(());
        },
        Err(e ) => {
            return Err(e)

        }

    }
}