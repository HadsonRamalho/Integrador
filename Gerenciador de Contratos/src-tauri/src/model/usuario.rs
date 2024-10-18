use crate::{controller::cria_pool, model::Pool};
use mysql_async::{params, prelude::Queryable};

use super::erro::MeuErro;

pub async fn atualiza_email(email_antigo: &str, email_novo: &str) -> Result<(), mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let resultado_conexao = conn.exec_drop("UPDATE usuarios SET email = :email_novo WHERE email = :email_antigo;", 
    params!{"email_novo" => email_novo, "email_antigo" => email_antigo}).await;
    match resultado_conexao{
        Ok(()) => {
            return Ok(());
        },
        Err(e ) => {
            println!("{:?}", e);
            return Err(mysql_async::Error::Other(Box::new(MeuErro::AtualizarEmailUsuario)))
        }
    }
}

pub async fn atualiza_senha(email: &str, senha_nova: &str) -> Result<(), mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let resultado_conexao = conn.exec_drop("UPDATE usuarios SET senha = :senha_nova WHERE email = :email;", 
    params!{"senha_nova" => senha_nova, "email" => email}).await;
    match resultado_conexao{
        Ok(()) => {
            return Ok(());
        },
        Err(e ) => {
            println!("{:?}", e);
            return Err(mysql_async::Error::Other(Box::new(MeuErro::AtualizarSenhaUsuario)));
        }
    }
}

pub async fn busca_id_usuario(email: &str) -> Result<String, mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let id_usuario: Option<String> = conn.exec_first("SELECT UUID FROM usuarios WHERE email = :email;", 
    params!{"email" => email}).await?;
    match id_usuario{
        None => {
            return Err(mysql_async::Error::Other(Box::new(MeuErro::EmailNaoEncontrado)));
        },
        Some(_) => {
            return Ok(id_usuario.unwrap());
        }
    }
}

pub async fn atualiza_nome(email: &str, nome: &str) -> Result<(), mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let resultado_conexao = conn.exec_drop("UPDATE usuarios SET nomecompleto = :nome WHERE email = :email;", 
    params!{"email" => email, "nome" => nome}).await;
    match resultado_conexao{
        Ok(()) => {
            return Ok(());
        },
        Err(e ) => {
            println!("{:?}", e);
            return Err(mysql_async::Error::Other(Box::new(MeuErro::AtualizarNomeUsuario)))
        }
    }
}

pub async fn busca_email_usuario(id: &str) -> Result<String, mysql_async::Error>{
    let pool = cria_pool().await?;
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

pub async fn busca_nome_usuario(pool: &Pool, id: &str) -> Result<String, mysql_async::Error>{
    let mut conn = pool.get_conn().await?;
    let nome_usuario: Option<String> = conn.exec_first("SELECT nomecompleto FROM usuarios WHERE UUID = :id;", 
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

pub async fn busca_cnpj_usuario(pool: &Pool, id: &str) -> Result<String, mysql_async::Error>{
    let mut conn = pool.get_conn().await?;
    let cnpj: Option<String> = conn.exec_first("SELECT cnpj FROM usuarios WHERE UUID = :id;", 
    params!{"id" => id}).await?;
    let server_error = mysql_async::ServerError{
        code: 1045, 
        message: "ID inválido.".to_string(),
        state: "28000".to_string()
    };
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

pub async fn busca_cpf_usuario(id: &str) -> Result<String, mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let cpf: Option<String> = conn.exec_first("SELECT cpf FROM usuarios WHERE UUID = :id;", 
    params!{"id" => id}).await?;
    match cpf{
        None => {
            return Err(mysql_async::Error::Other(Box::new(MeuErro::CpfNaoEncontrado)));
        },
        Some(cpf) => {
            if cpf.is_empty(){
                return Err(mysql_async::Error::Other(Box::new(MeuErro::CpfNaoEncontrado)));
            }
            return Ok(cpf);
        }
    }
}

pub async fn deleta_conta(idusuario: String, email: String) -> Result <(), mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let id = idusuario.clone();
    let cpf = busca_cpf_usuario(&idusuario).await?;
    let _atualizacao_socioadm = conn.exec_drop("UPDATE socioadm SET sociostatus = 0 WHERE cpf = :cpf;", 
    params!("cpf" => cpf)).await?;
    let resultado = conn.exec_drop("DELETE FROM usuarios WHERE UUID = :id AND email = :email;", 
    params! {"id" => id, "email" => email}).await;
    match resultado{
        Ok(_) => {return Ok(())},
        Err(e) => {return Err(mysql_async::Error::Other(Box::new(e)))}
    }
}