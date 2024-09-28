use mysql_async::prelude::Queryable;
use mysql_async::{params, Pool};

use crate::model::erro::MeuErro;
use crate::model::usuario::busca_id_usuario;
use crate::model::{self, usuario};
use crate::controller::valida_email;
use crate::controller;
use super::{cria_pool, formata_cpf, gera_hash, verifica_hash};

#[tauri::command]
pub async fn cria_conta(
    nome_completo: &str,
    email: &str,
    senha1: &str,
    senha2: &str,
    cpf: &str,
    cnpj: &str
) -> Result<(), String> {
    let email = email.trim(); // Removendo todos os espaços em branco do email
    let cpf = cpf.trim();
    if cpf.trim().is_empty(){
        return Err("Erro: O CPF não pode estar vazio".to_string());
    }
    let cpf = formata_cpf(cpf)?;
    if !valida_email(&email) {
        return Err("Erro: E-mail inválido. Deve conter '@' e '.'".to_string());
    }
    if senha1.trim() != senha2.trim() {
        return Err("Erro: As senhas são diferentes".to_string()); // Conta não criada
    }
    let cnpj = controller::locadora::formata_cnpj(cnpj)?;
    match valida_senha(senha1) {
        Ok(_) => {}
        Err(e) => {
            return Err(e);
        }
    }
    let hash = gera_hash(senha1); // Criptografando a senha (Standard *BSD hash)
    let mut usuario =
        model::Usuario::novo_usuario(nome_completo.to_string(), email.to_string(), hash); // Cria um novo usuário
    if usuario.ja_cadastrado().await {
        return Err("Erro: Usuário já cadastrado".to_string());
    }
    let resultado_cadastro = cadastra_usuario(nome_completo, &email, usuario.get_hash(), &cpf, &cnpj).await;
    match resultado_cadastro {
        Ok(_) => return Ok(()),
        Err(_) => return Err("Erro no cadastro do usuário.".to_string()),
    }
}


pub async fn _verifica_senha(email: &str, senha: &str) -> Result<model::Usuario, String> {
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e.to_string())
        }
    };
    let usuario_autenticado = model::verifica_senha(&pool, &email, senha)
        .await
        .map_err(|e| format!("{}", e))?;
    Ok(usuario_autenticado)
}

pub async fn cadastra_usuario(nome: &str, email: &str, senha: &str, cpf: &str, cnpj: &str) -> Result<(), String> {
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err(e.to_string())
        }
    };
    let cpf = controller::formata_cpf(cpf)?;
    let _resultado_criacao = model::cadastra_usuario(&pool, nome, &email, senha, &cpf, cnpj)
        .await
        .map_err(|e| format!("{}", e))?;
    Ok(())
}


#[tauri::command]
pub async fn verifica_senha(email: &str, senha: &str) -> Result<(), String> {
    let senha = senha.trim();
    if senha.is_empty() {
        return Err("A senha não pode estar vazia".to_string());
    }
    let email = email.trim();
    if email.is_empty(){
        return Err("O e-mail não pode estar vazio".to_string());
    }
    let resultado_verificacao: Result<model::Usuario, String> = _verifica_senha(email, &senha).await;
    match resultado_verificacao {
        Ok(_) => return Ok(()),
        Err(e) => {
            return Err(e.to_string());
        }
    }
}


#[tauri::command]
pub async fn atualiza_email(email_antigo: String, email: String) -> Result<(), String>{
    let email: &str = email.trim();
    if !valida_email(email){
        return Err("Erro: Novo e-mail inválido".to_string())
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
                return Err("E-mail antigo inválido.".to_string()) 
            }
        },
        Err(_e) => {
            println!("{:?}", _e);
            return Err("Erro ao atualizar o e-mail".to_string());
        }
    }
 
    let r: Result<(), mysql_async::Error> = model::usuario::atualiza_email(&pool, &email_antigo, email).await;
    match r{
        Ok(()) => {
            return Ok(())
        },
        Err(_e) => {
            println!("Erro ao atualizar o e-mail");
            return Err("Erro ao atualizar o e-mail".to_string());
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
                return Err("Erro: E-mail inválido".to_string())
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
        return Err("Erro ao validar o token: E-mail inválido.".to_string());
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
    if email.trim().is_empty(){
        return Err("Erro: O e-mail está vazio".to_string())
    }
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
    if id.trim().is_empty(){
        return Err("Erro: O ID está vazio".to_string())
    }
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
    if id.trim().is_empty(){
        return Err("Erro: O ID está vazio".to_string())
    }
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

#[tauri::command]
pub async fn busca_cnpj_usuario(id: String) -> Result<String, String>{
    if id.trim().is_empty(){
        return Err("Erro: O ID está vazio".to_string())
    }
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

#[tauri::command]
pub async fn atualiza_nome(email: &str, nome: &str) -> Result<(), String>{
    let email = email.trim();
    if email.is_empty() {
        return Err(MeuErro::EmailVazio.to_string());
    }
    if nome.is_empty(){
        return Err("Erro: Nome vazio".to_string());
    }
    let resultado_atualizacao: Result<(), mysql_async::Error> = model::usuario::atualiza_nome(email, nome).await;
    match resultado_atualizacao{
        Ok(()) => {
            return Ok(())
        },
        Err(_e) => {
            println!("Erro ao atualizar o nome");
            return Err("Erro ao atualizar o nome".to_string());
        }
    }
}

#[tauri::command]
pub async fn deleta_conta(idusuario: String, email: String) -> Result<(), String>{
    if idusuario.trim().is_empty() || email.trim().is_empty(){
        return Err(MeuErro::CamposVazios.to_string())
    }
    let resultado_delete = _deleta_conta(idusuario, email).await;
    match resultado_delete{
        Ok(_) => {return Ok(())},
        Err(e) => {return Err(e.to_string())}
    }
}

pub async fn _deleta_conta(idusuario: String, email: String) -> Result <(), mysql_async::Error>{
    let pool = cria_pool().await?;
    let mut conn = pool.get_conn().await?;
    let resultado = conn.exec_drop("DELETE FROM usuarios WHERE UUID = :id AND email = :email;", 
    params! {"id" => idusuario, "email" => email}).await;
    match resultado{
        Ok(_) => {return Ok(())},
        Err(e) => {return Err(mysql_async::Error::Other(Box::new(e)))}
    }
}