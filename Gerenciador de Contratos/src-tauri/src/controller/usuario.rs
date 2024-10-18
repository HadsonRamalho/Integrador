use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::model::erro::MeuErro;
use crate::model::usuario::busca_id_usuario;
use crate::model::{self, usuario};
use crate::controller::checa_email;
use crate::controller;
use super::{cria_pool, formata_cpf, gera_hash, verifica_hash, EmailInput};

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

pub async fn cria_conta(
    Json(input): Json<UsuarioInput>
)  -> Result<StatusCode, (StatusCode, String)>  {
    let mut usuario = input;
    if usuario.nome.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, format!("{}", MeuErro::NomeVazio)));
    }
    if usuario.cpf.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, format!("{}", MeuErro::CpfVazio)));
    }
    let cpf = match formata_cpf(&usuario.cpf){
        Ok(cpf) => {cpf},
        Err(e) => {
            return Err((StatusCode::BAD_REQUEST, format!("{}", format!("{}", e))));
        }
    };
    
    match controller::usuario::checa_email(Json( EmailInput{email: usuario.email.clone()} )).await{
        Ok(_) => {},
        Err(e) => {
            return Err(e)
        }
    }

    if usuario.senha1.trim() != usuario.senha2.trim() {
        return Err((StatusCode::BAD_REQUEST, format!("{}", MeuErro::SenhasDiferentes)));
    }
    let cnpj: String = match controller::locadora::formata_cnpj(&usuario.cnpj){
        Ok(cnpj) => {cnpj}
        Err(e) => {
            return Err((StatusCode::BAD_REQUEST, format!("{}", e)));
        }
    };
    match valida_senha(&usuario.senha1) {
        Ok(_) => {}
        Err(e) => {
            return Err((StatusCode::BAD_REQUEST, format!("{}", e)))
        }
    }
    let hash = gera_hash(&usuario.senha1); // Criptografando a senha (Standard *BSD hash)

    usuario.senha1 = hash.clone();
    usuario.senha2 = hash;
    usuario.cnpj = cnpj;
    usuario.cpf = cpf;

    let novousuario: model::Usuario = usuario.into();
    if novousuario.ja_cadastrado().await {
        return Err((StatusCode::BAD_REQUEST, "Esse e-mail já pertence a outra conta".to_string()));
    }
    let resultado_cadastro = model::salva_usuario(novousuario).await;
    match resultado_cadastro {
        Ok(_) => return Ok(StatusCode::OK),
        Err(e) => return Err((StatusCode::BAD_REQUEST, format!("Erro no cadastro do usuário: {}", e))),
    }
}

#[derive(Deserialize, Serialize)]
pub struct VerificaSenhaInput{
    pub email: String,
    pub senha: String
}

pub async fn verifica_senha(input: Json<VerificaSenhaInput>) -> Result<StatusCode, (StatusCode, Json<String>)>  {
    let senha = input.senha.clone();
    let email = input.email.clone();

    let senha = senha.trim();
    if senha.is_empty() {
        return Err((StatusCode::BAD_REQUEST, Json("A senha não pode estar vazia".to_string())));
    }
    let email = email.trim();
    if email.is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("O e-mail não pode estar vazio".to_string())));
    }
    match model::verifica_senha(email, senha).await{
        Ok(_) => {
            return Ok(StatusCode::OK)
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e.to_string())))
        }
    };
}

#[derive(Deserialize, Serialize)]
pub struct AtualizaEmailInput{
    pub email_antigo: String,
    pub email_novo: String
}

pub async fn atualiza_email(input: Json<AtualizaEmailInput>) -> Result<StatusCode, (StatusCode, String)>{
    let email = input.email_novo.clone();
    let email_antigo = input.email_antigo.clone();

    let email: &str = email.trim();
    
    match checa_email(Json(EmailInput{email: email.to_string()})).await{
        Ok(_) => {},
        Err(e) => return Err(e)
    }

    let resultado_busca: Result<String, mysql_async::Error> = model::busca_email(&email_antigo).await;
    match resultado_busca{
        Ok(_) => {},
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }

    let resultado_busca2: bool = model::busca_email(&email).await.is_err();
    if !resultado_busca2{
        return Err((StatusCode::BAD_REQUEST, "O e-mail novo já pertence a outro usuário.".to_string()))
    }   
    
    let resultado_atualizacao: Result<(), mysql_async::Error> = model::usuario::atualiza_email( &email_antigo, email).await;
    match resultado_atualizacao{
        Ok(()) => {
            return Ok(StatusCode::OK)
        },
        Err(e) => {
            println!("Erro ao atualizar o e-mail");
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
        }
    }
}

pub async fn atualiza_senha(input: Json<VerificaSenhaInput>) -> Result<StatusCode, (StatusCode, String)>{
    let email = input.email.clone();
    let nova_senha = input.senha.clone();
    match valida_senha(&nova_senha){
        Ok(_) => {},
        Err(e) => {
            return Err((StatusCode::BAD_REQUEST, format!("{}", e)))
        }
    }
    let nova_senha = gera_hash(nova_senha.trim());

    let resultado_busca: Result<String, mysql_async::Error> = model::busca_email(&email).await;
    let email = match resultado_busca{
        Ok(email) => {
            email
        },
        Err(e) => {
            return Err((StatusCode::BAD_REQUEST, format!("{}", e)))
        }
    };

    match checa_email(Json(EmailInput{email: email.clone()})).await{
        Ok(_) => {},
        Err(e) => {
            return Err(e)
        }
    };

    let resultado_atualizacao: Result<(), mysql_async::Error> = model::usuario::atualiza_senha(&email, &nova_senha).await;
    match resultado_atualizacao{
        Ok(()) => {
            return Ok(StatusCode::OK)
        },
        Err(_e) => {
            println!("Erro ao atualizar a senha");
            return Err((StatusCode::BAD_REQUEST, format!("{}", MeuErro::AtualizarSenhaUsuario)))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct VerificaTokenInput{
    pub email: String,
    pub token: String
}

pub async fn verifica_token(input: Json<VerificaTokenInput>) -> Result<StatusCode, (StatusCode, String)>{
    let email = input.email.clone();
    let token = input.token.clone();

    let email = email.trim();
    let token = token.trim();

    let token = token.to_string().replace("\"", "");
    
    match checa_email(Json(EmailInput{email: email.to_string()})).await{
        Ok(_) => {},
        Err(e) => {
            return Err(e)
        }
    }

    let id = busca_id_usuario(email).await;
    let uid;
    match id{
        Ok(id) =>{
            if id.is_empty(){
                return Err((StatusCode::BAD_REQUEST, "Erro ao validar o token: Verifique o email.".to_string()))
            }
            uid = id;
        },
        Err(e) =>{
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
    
    let email = model::usuario::busca_email_usuario(&token).await;
    match email{
        Ok(_) =>{
            if verifica_hash(&email.unwrap(), uid){
                return Ok(StatusCode::OK);
            }
            return Err((StatusCode::BAD_REQUEST, "Token inválido.".to_string()));
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))            
        }
    }
}

//#[tauri::command]
pub async fn busca_id(input: Json<EmailInput>) -> Result<(StatusCode, Json<String>), (StatusCode, String)>{
    match checa_email(Json(EmailInput{email: input.email.clone()})).await{
        Ok(_) => {},
        Err(e) =>{
            return Err(e)
        }
    }

    let email = input.email.clone();

    let resultado_busca = usuario::busca_id_usuario(&email).await;
    match resultado_busca{
        Ok(id) =>{
            if id.is_empty(){
                return Err((StatusCode::BAD_REQUEST, "ID não encontrado. Verifique o e-mail.".to_string()));
            }
            return Ok((StatusCode::OK, Json(id)))
        },
        Err(e) =>{
            return Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()));
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

//#[tauri::command]
pub async fn busca_email_usuario(input: Json<String>) -> Result<(StatusCode, Json<String>), (StatusCode, String)>{
    let id = input.0;
    if id.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, "O ID está vazio.".to_string()))
    }
    let resultado_busca = model::usuario::busca_email_usuario(&id).await;
    match resultado_busca{
        Ok(email) => {
            return Ok((StatusCode::OK, Json(email)))
    }, Err(e) => {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }
    }
}



//#[tauri::command]
pub async fn busca_nome_usuario(input: Json<String>) -> Result<(StatusCode, Json<String>), (StatusCode, String)>{
    let id = input.0;
    if id.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, "Erro: O ID está vazio".to_string()))
    }
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err((StatusCode::SERVICE_UNAVAILABLE, e.to_string()))
        }
    };
    let resultado_busca = model::usuario::busca_nome_usuario(&pool, &id).await;
    match resultado_busca{
        Ok(nome) => {
            return Ok((StatusCode::OK, Json(nome)))
    }, Err(e) => {
            return Err((StatusCode::BAD_REQUEST, e.to_string()));
        }
    }
}

//#[tauri::command]
pub async fn busca_cnpj_usuario(input: Json<String>) -> Result<(StatusCode, Json<String>), (StatusCode, String)>{
    let id = input.0;
    if id.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, MeuErro::CamposVazios.to_string()))
    }
    let pool = match controller::cria_pool().await {
        Ok(pool) => {
            pool
        }, 
        Err(e) =>{
            return Err((StatusCode::SERVICE_UNAVAILABLE, format!("{}", e)))
        }
    };
    let resultado_busca = model::usuario::busca_cnpj_usuario(&pool, &id).await;
    match resultado_busca{
        Ok(cnpj) => { 
            return Ok((StatusCode::OK, Json(cnpj)))
        }, 
        Err(_) => {
            return Err((StatusCode::BAD_REQUEST, MeuErro::CnpjNaoEncontrado.to_string()));
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AtualizaNomeInput{
    pub nome: String,
    pub email: String
}

//#[tauri::command]
pub async fn atualiza_nome(input: Json<AtualizaNomeInput>) -> Result<StatusCode, (StatusCode, String)>{
    let email = input.email.clone();
    let nome = input.nome.clone();
    let email = email.trim();
    if email.is_empty() {
        return Err((StatusCode::BAD_REQUEST, format!("{}", MeuErro::EmailVazio)))
    }
    if nome.is_empty(){
        return Err((StatusCode::BAD_REQUEST, format!("{}", MeuErro::NomeVazio)))
    }
    let resultado_busca = busca_id_usuario( email).await.is_ok();
    if !resultado_busca{
        return Err((StatusCode::BAD_REQUEST, format!("{}", MeuErro::EmailNaoEncontrado)))      
    }
    let resultado_atualizacao: Result<(), mysql_async::Error> = model::usuario::atualiza_nome(email, &nome).await;
    match resultado_atualizacao{
        Ok(()) => {
            return Ok(StatusCode::OK)
        },
        Err(e) => {
            println!("{:?}", e);
            return Err((StatusCode::BAD_REQUEST, format!("{}", e)))
        }
    }

}

#[derive(Serialize, Deserialize)]
pub struct DeletaContaInput{
    pub email: String,
    pub idusuario: String
}

//#[tauri::command]
pub async fn deleta_conta(input: Json<DeletaContaInput>) -> Result<(), (StatusCode, String)>{
    let idusuario = input.idusuario.clone();
    let email = input.email.clone();
    if idusuario.trim().is_empty() || email.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, MeuErro::CamposVazios.to_string()))
    }
    let resultado_delete = model::usuario::deleta_conta(idusuario, email).await;
    match resultado_delete{
        Ok(_) => {return Ok(())},
        Err(e) => {return Err((StatusCode::BAD_REQUEST, e.to_string()))}
    }
}

