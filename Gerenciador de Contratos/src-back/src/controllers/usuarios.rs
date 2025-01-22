use axum::{extract::Query, http::StatusCode, Json};
use pwhash::unix::verify;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::models::{self, usuarios::Usuario};

use super::{cria_conn, envia_emails::envia_email_codigo, formata_cnpj, formata_cpf, gera_hash};

#[derive(Deserialize, Serialize, Clone, ToSchema)]
pub struct UsuarioInput{
    pub nome: String,
    pub email: String,
    pub senha: String,
    pub documento: String
}

#[derive(Deserialize, Validate)]
pub struct EmailInput{
    #[validate(email)]
    pub email: String
}


#[utoipa::path(
    post,
    path = "/cadastra_usuario",
    responses(
        (
            status = 200, 
            description = "Dados válidos. O usuário foi cadastrado.",
            body = UserId
        ),
        (
            status = 500,
            description = "Erro ao cadastrar o usuário."
        ),
        (
            status = 400,
            description = "Algum dos campos inseridos está incorreto."
        ),
    ),
    request_body = UsuarioInput    
)]

pub async fn cadastra_usuario(usuario: Json<UsuarioInput>)
    -> Result<(StatusCode, Json<UserId>), (StatusCode, Json<String>)>{
    match valida_usuario(&usuario.0).await{
        Ok(_) => {},
        Err(e) => {
            println!("{:?}", e);
            return Err((StatusCode::BAD_REQUEST, Json(e)))
        }
    }    

    let email_clone = usuario.email.to_string();
    let senha = gera_hash(&usuario.senha);

    let idusuario = gera_hash(&usuario.email);
    let idusuario_clone = idusuario.clone();
    let now = chrono::Utc::now().naive_utc();
    let usuario = Usuario{
        nome: usuario.nome.to_string(),
        email: usuario.email.to_string(),
        senha,
        documento: usuario.documento.to_string(),
        datacadastro: now,
        idusuario
    };

    let conn = &mut cria_conn()?;

    match models::usuarios::cadastra_usuario(conn, usuario).await{
        Ok(_) => {
            
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }

    match envia_email_codigo(email_clone, "ativação de conta").await{
        Ok(codigoativacao) => {
            println!("Código de ativação: {}", codigoativacao.1.0);
            return Ok((StatusCode::OK, Json(UserId{idusuario: idusuario_clone})))
        },
        Err(e) => {
            return Err(e)
        }
    }

}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CredenciaisUsuario{
    pub email: String,
    pub senha: String
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UserId{
    pub idusuario: String
}

#[utoipa::path(
    post,
    path = "/realiza_login",
    responses(
        (
            status = 200, 
            description = "Credenciais verificadas e válidas. Login pode ser realizado.",
            body = UserId       
        ),
        (
            status = 500,
            description = "O e-mail inserido não está registrado no sistema."
        ),
        (
            status = 400,
            description = "Algum dos campos inseridos está incorreto."
        ),
    ),
    request_body = CredenciaisUsuario    
)]

pub async fn realiza_login(input: Json<CredenciaisUsuario>)
    -> Result<(StatusCode, Json<UserId>), (StatusCode, Json<String>)>{
    let email = input.email.to_string();
    let senha: String = input.senha.to_string();

    if senha.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("A senha está vazia.".to_string())))
    }
    
    match valida_email(Json(EmailInput{
        email: email.clone()
    })).await{
        Ok(_) => {},
        Err(e) => {
            return Err(e)
        }
    }

    let hash_senha = match busca_senha_usuario(Json(EmailInput{
        email: email.clone()
    })).await{
        Ok(hash) => {hash},
        Err(e) => {
            return Err(e)
        }
    };
    let hash_senha = hash_senha.1.to_string();
    
    let id = busca_usuario_email(Query(EmailInput{
        email
    })).await?.1.0;

    if verify(senha, &hash_senha){
        return Ok((StatusCode::OK, Json(UserId{
            idusuario: id
        })))
    }
    return Err((StatusCode::BAD_REQUEST, Json("Erro no login.".to_string())))
}

pub async fn busca_senha_usuario(email: Json<EmailInput>) 
    -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    let email_clone = email.email.clone();
    match valida_email(email).await{
        Ok(_) => {},
        Err(e) => {
            return Err(e)
        }
    }

    let conn = &mut cria_conn()?;

    match models::usuarios::busca_senha_usuario(conn, email_clone).await{
        Ok(hash) => {
            return Ok((StatusCode::OK, Json(hash)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}

pub async fn busca_email_usuario(input: Json<String>) -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    let id = input.0;
    if id.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("O ID está vazio.".to_string())))
    }
    
    let conn = &mut cria_conn()?;

    let resultado_busca = models::usuarios::busca_email_usuario(conn, id).await;
    match resultado_busca{
        Ok(email) => {
            return Ok((StatusCode::OK, Json(email)))
    }, Err(e) => {
        return Err((StatusCode::BAD_REQUEST, Json(e.to_string())));
    }
    }
}

pub fn valida_senha(senha: &str) -> Result<(), String>{
    if senha.trim().is_empty(){
        return Err("A senha está vazia.".to_string())
    }
    if senha.len() < 8{
        return Err("A senha é muito curta.".to_string())
    }
    if !senha.chars().any(|c| c.is_ascii_digit()){
        return Err("A senha deve conter ao menos um número".to_string())
    }
    if !senha.chars().any(|c| c.is_ascii_punctuation()){
        return Err("A senha deve conter ao menos um símbolo".to_string())
    }
    if !senha.chars().any(|c| c.is_uppercase()){
        return Err("A senha deve conter ao menos uma letra maiúscula.".to_string())
    }
    return Ok(())
}

pub async fn valida_usuario(usuario: &UsuarioInput) -> Result<(), String>{
    let nome = usuario.nome.to_string();
    if nome.trim().is_empty(){
        return Err("Erro ao validar o e-mail.".to_string())
    }

    let email = usuario.email.to_string();
    match valida_email(Json(EmailInput{email: email.clone()})).await{
        Ok(_) => {},
        Err(e) => {
            return Err(e.1.0)
        }
    }

    let senha = usuario.senha.to_string();
    match valida_senha(&senha){
        Ok(()) => {
        },
        Err(e) => {
            return Err(e)
        }
    }

    let documento_ = usuario.documento.to_string();
    match formata_documento(&documento_){
        Ok(_) => {
            
        },
        Err(e) => {
            return Err(e)
        }
    };
    return Ok(())
}

#[derive(Deserialize, Serialize)]
pub struct AtualizaEmailInput{
    pub email_antigo: String,
    pub email_novo: String,
    pub senha: String
}


pub async fn atualiza_email_usuario(input: Json<AtualizaEmailInput>) -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    let email_antigo = input.email_antigo.to_string();
    let email_novo = input.email_novo.to_string();

    match valida_email(Json(
        EmailInput{
            email: email_antigo.clone()
        }
    )).await{
        Ok(_) => {},
        Err(e) => {
            return Err(e)
        }
    }

    match valida_email(Json(
        EmailInput{
            email: email_novo.clone()
        }
    )).await{
        Ok(_) => {},
        Err(e) => {
            return Err(e)
        }
    }

    match realiza_login(Json(CredenciaisUsuario{
        email: email_antigo.clone(),
        senha: input.senha.to_string()
    })).await{
        Ok(_) => {},
        Err(e) => {
            return Err(e)
        }
    }
    
    match busca_usuario_email(Query(EmailInput{
        email: email_antigo.clone()
    })).await{
        Ok(_) => {
        },
        Err(e) => {
            return Err(e)
        }
    }
    
    match busca_usuario_email(Query(EmailInput{
        email: email_novo.clone()
    })).await{
        Ok(_) => {
            return Err((StatusCode::BAD_REQUEST, Json("Esse e-mail pertence a outro usuário.".to_string())))
        },
        Err(_) => {
            
        }
    }

    let conn = &mut cria_conn()?;

    match models::usuarios::atualiza_email_usuario(conn, email_antigo, email_novo).await{
        Ok(email_atualizado) => {
            return Ok((StatusCode::OK, Json(email_atualizado)))
        },
        Err(e) =>{
            return Err((StatusCode::BAD_REQUEST, Json(e)))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct AtualizaSenhaInput{
    pub email: String,
    pub senha_antiga: String,
    pub senha_nova: String
}

pub async fn atualiza_senha_usuario(input: Json<AtualizaSenhaInput>)
    -> Result<StatusCode, (StatusCode, Json<String>)>{
    let email = input.email.to_string();
    match valida_email(Json(EmailInput{
        email: email.clone()
    })).await{
        Ok(_) => {},
        Err(e) => {
            return Err(e)
        }
    }

    match realiza_login(Json(CredenciaisUsuario{
        email: email.clone(),
        senha: input.senha_antiga.to_string()
    })).await{
        Ok(_) => {},
        Err(e) => {
            return Err(e)
        }
    }

    let senha_nova = input.senha_nova.to_string();
    match valida_senha(&senha_nova){
        Ok(_) => {},
        Err(e) => {
            return Err((StatusCode::BAD_REQUEST, Json(e)))
        }
    }
    let senha_nova = gera_hash(&senha_nova);

    let conn = &mut cria_conn()?;

    match models::usuarios::atualiza_senha_usuario(conn, email, senha_nova).await{
        Ok(_) => {
            return Ok(StatusCode::OK)
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}

pub async fn busca_usuario_email(Query(params): Query<EmailInput>) -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)>{
    match valida_email(Json(EmailInput{
        email: params.email.clone()
    })).await{
        Ok(_) => {},
        Err(e) => {
            return Err(e)
        }
    }

    let email = params.email.to_string();
    
    let conn = &mut cria_conn()?;

    let res = models::usuarios::busca_usuario_email(conn, email).await;
    match res{
        Ok(idusuario) => {
            return Ok((StatusCode::OK, Json(idusuario)))
        },
        Err(e) => {
            return Err((StatusCode::BAD_REQUEST, Json(e)))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct IdInput{
    pub id: String
}

pub async fn busca_usuario_id(Query(params): Query<IdInput>)
    -> Result<(StatusCode, Json<Usuario>), (StatusCode, Json<String>)>{
    if params.id.trim().is_empty(){
        return Err((StatusCode::BAD_REQUEST, Json("Um ou mais campos estão vazios.".to_string())))
    }
    let id = params.id.to_string();
    let conn = &mut cria_conn()?;
    match models::usuarios::busca_usuario_id(conn, id).await{
        Ok(usuario) => {
            return Ok((StatusCode::OK, Json(usuario)))
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }
}

pub async fn valida_email(input: Json<EmailInput>) -> Result<(StatusCode, Json<String>), (StatusCode, Json<String>)> {
    match input.validate(){
         Ok(_) => {
             return Ok((StatusCode::OK, Json(input.0.email)))
         },
         Err(e) => {
             return Err((StatusCode::BAD_REQUEST, Json(e.to_string())))
         }
     }
}

pub fn formata_documento(documento_: &str) -> Result<String, String>{
    match formata_cpf(documento_){
        Ok(cpf) => {
            return Ok(cpf)
        },
        Err(_) => {
        }
    }
    match formata_cnpj(documento_){
        Ok(cnpj) => {
            return Ok(cnpj)
        },
        Err(_) => {
            return Err("O documento não é válido.".to_string())
        }
    }
}