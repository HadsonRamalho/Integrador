use crate::{controller::cria_pool, model::Pool};
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
            return Err(e);

        }

    }
}

pub async fn busca_id_usuario(pool: &Pool, email: &str) -> Result<String, mysql_async::Error>{
    let mut conn = pool.get_conn().await?;
    let id_usuario: Option<String> = conn.exec_first("SELECT UUID FROM usuarios WHERE email = :email;", 
    params!{"email" => email}).await?;
    let server_error = mysql_async::ServerError{
        code: 1045, //Código de erro
        message: "ID não encontrado".to_string(),
        state: "28000".to_string()
    };
    match id_usuario{
        None => {
            return Err(mysql_async::Error::Server(server_error));
        },
        Some(_) => {
            return Ok(id_usuario.unwrap());

        }

    }
}

pub async fn verifica_id_usuario(pool: &Pool, id: &str) -> Result<(), mysql_async::Error>{
    let mut conn = pool.get_conn().await?;
    let resultado_conexao = conn.exec_drop("SELECT UUID FROM usuarios WHERE UUID = :id",
    params! {"id" => id}).await;
    match resultado_conexao{
        Ok(())=>{
            return Ok(());
        },
        Err(e) => {
            return Err(e);
        }
    }



}

pub async fn atualiza_nome(email: &str, nome: &str) -> Result<(), mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let resultado_conexao = conn.exec_drop("UPDATE usuarios SET nome_completo = :nome WHERE email = :email;", 
    params!{"email" => email, "nome" => nome}).await;
    match resultado_conexao{
        Ok(()) => {
            return Ok(());
        },
        Err(e ) => {
            return Err(e)
        }
    }
}