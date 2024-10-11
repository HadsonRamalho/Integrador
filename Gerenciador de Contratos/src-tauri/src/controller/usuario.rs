use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::model::erro::MeuErro;
use crate::model::usuario::busca_id_usuario;
use crate::model::{self, usuario, Usuario};
use crate::controller::valida_email;
use crate::controller;
use super::{formata_cpf, gera_hash, verifica_hash};

#[derive(Deserialize)]
pub struct UsuarioInput {
    // Objeto de usuário para unificar dados
    pub nome: String,
    pub email: String,
    pub senha1: String,
    pub senha2: String,
    pub cpf: String,
    pub cnpj: String
}

#[derive(Serialize, Debug)]
pub struct MyErrorResponse {
    error: String,
}

#[derive(Serialize, Debug)]
pub struct MyResponse {
    message: String,
}


// Definindo um enum para encapsular os possíveis retornos
#[derive(Debug)]
pub enum MyResult {
    Success(MyResponse),
    Error(MyErrorResponse),
}

impl IntoResponse for MyResult {
    fn into_response(self) -> axum::response::Response {
        match self {
            MyResult::Success(res) => (StatusCode::OK, Json(res)).into_response(),
            MyResult::Error(err) => (StatusCode::BAD_REQUEST, Json(err)).into_response(),
        }
    }
}

pub async fn cria_conta(
    Json(input): Json<UsuarioInput>
)  -> MyResult  {
    let usuario = input;
    let email = usuario.email.trim(); // Removendo todos os espaços em branco do email
    let cpf = usuario.cpf.trim();
    if cpf.trim().is_empty(){
        return MyResult::Error(controller::usuario::MyErrorResponse { error: "Erro no CPF".to_string() });
    }
    let cpf = match formata_cpf(cpf){
        Ok(cpf) => {cpf},
        Err(e) => {
            return MyResult::Error(controller::usuario::MyErrorResponse { error: format!("{}", e) });
        }
    };
    if !valida_email(&email) {
        return MyResult::Error(controller::usuario::MyErrorResponse { error: "Erro no E-mail".to_string() });
    }
    if usuario.senha1.trim() != usuario.senha2.trim() {
        return MyResult::Error(controller::usuario::MyErrorResponse { error: "Erro nas senhas".to_string() });
    }
    let cnpj = controller::locadora::formata_cnpj(&usuario.cnpj).unwrap();
    match valida_senha(&usuario.senha1) {
        Ok(_) => {}
        Err(e) => {
            return MyResult::Error(controller::usuario::MyErrorResponse { error: format!("{}", e) });
        }
    }
    let hash = gera_hash(&usuario.senha1); // Criptografando a senha (Standard *BSD hash)
    let mut novousuario =
        model::Usuario::novo_usuario(usuario.nome.to_string(), email.to_string(), hash); // Cria um novo usuário
    if novousuario.ja_cadastrado().await {
        return MyResult::Error(controller::usuario::MyErrorResponse { error: "Já cadastrado".to_string() });
    }
    let resultado_cadastro = cadastra_usuario(&usuario.nome, &email, novousuario.get_hash(), &cpf, &cnpj).await;
    match resultado_cadastro {
        Ok(_) => return MyResult::Success(controller::usuario::MyResponse {message: "".to_string()}),
        Err(_) => return MyResult::Error(controller::usuario::MyErrorResponse { error: "Erro no cadastro".to_string() }),
    }
}

fn usuario_vazio() -> Usuario{
    let u = Usuario{
        nome: "".to_string(),
        email: "".to_string(),
        senha: "".to_string(),
    };
    u
}

#[derive(Deserialize, Serialize)]
pub struct VerificaSenhaInput{
    pub email: String,
    pub senha: String
}

pub async fn _verifica_senha(email: String, senha: String) -> (StatusCode, axum::Json<Usuario>) {
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            println!("{:?}", e);
            return (StatusCode::SERVICE_UNAVAILABLE, axum::Json(usuario_vazio()))
        }
    };    
    let usuario_autenticado = model::verifica_senha(&pool, &email, &senha)
        .await
        .map_err(|e| format!("{}", e));
    let usuario_autenticado = match usuario_autenticado{
        Ok(u) => {u},
        Err(e) => {
            println!("{:?}", e);
            return (StatusCode::SERVICE_UNAVAILABLE, axum::Json(usuario_vazio()))}
    };
    (StatusCode::OK, axum::Json(usuario_autenticado))
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

pub async fn verifica_senha(input: Json<VerificaSenhaInput>) -> MyResult {
    let email = input.email.clone();
    let senha = input.senha.clone();
    let senha = senha.trim();
    if senha.is_empty() {
        return MyResult::Error(controller::usuario::MyErrorResponse { error:"Senha tá vazia".to_string() });
    }
    let email = email.trim();
    if email.is_empty(){
        return MyResult::Error(controller::usuario::MyErrorResponse { error: "E-mail tá vazio".to_string() })
    }
    let resultado_verificacao = _verifica_senha(email.to_string(), senha.to_string()).await;
    let status = resultado_verificacao.0;
    if status != StatusCode::OK{
        return MyResult::Error(controller::usuario::MyErrorResponse { error: "Erro na verificação".to_string() })
    }
    return MyResult::Success(controller::usuario::MyResponse { message: "Sucesso no cadastro".to_string() })
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
    let resultado_busca2: bool= model::busca_email(&pool, &email).await.is_err();
    if !resultado_busca2{
        return Err("E-mail pertence a outro usuário".to_string())
    }
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
    
    let email = model::usuario::busca_email_usuario(&pool, token).await;
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
    let email = model::usuario::busca_email_usuario(&pool, &id).await;
    match email{
        Ok(_) => { return Ok(email.unwrap());
    }, Err(e) => {
        return Err(e.to_string());
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
    let nome = model::usuario::busca_nome_usuario(&pool, &id).await;
    match nome{
        Ok(_) => { return Ok(nome.unwrap());
    }, Err(e) => {
        return Err(e.to_string());
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
    let cnpj = model::usuario::busca_cnpj_usuario(&pool, &id).await;
    match cnpj{
        Ok(_) => { 
            return Ok(cnpj.unwrap());
        }, 
        Err(e) => {
            return Err(e.to_string());
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
        return Err(MeuErro::NomeVazio.to_string());
    }
    let resultado_atualizacao: Result<(), mysql_async::Error> = model::usuario::atualiza_nome(email, nome).await;
    match resultado_atualizacao{
        Ok(()) => {
            return Ok(())
        },
        Err(e) => {
            println!("{:?}", e);
            return Err(e.to_string());
        }
    }
}

#[tauri::command]
pub async fn deleta_conta(idusuario: String, email: String) -> Result<(), String>{
    if idusuario.trim().is_empty() || email.trim().is_empty(){
        return Err(MeuErro::CamposVazios.to_string())
    }
    let resultado_delete = model::usuario::deleta_conta(idusuario, email).await;
    match resultado_delete{
        Ok(_) => {return Ok(())},
        Err(e) => {return Err(e.to_string())}
    }
}

