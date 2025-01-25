use axum::{extract::{Query, State}, Json};
use hyper::StatusCode;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use std::{env, ops::Deref, sync::Arc};

use crate::{controllers::usuarios::UserId, models::{self, usuarios::Usuario}};

use super::{codigos_recuperacao::gera_codigo_recuperacao, cria_conn, envia_emails::envia_email_codigo, gera_hash, usuarios::{busca_usuario_email, busca_usuario_email_oauth, valida_email, EmailInput}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub google_oauth_client_id: String,
    pub google_oauth_client_secret: String,
    pub google_oauth_redirect_url: String,
}

impl Config {
    pub fn init() -> Arc<Self> {
        use dotenv::dotenv;
        dotenv().ok();
        let google_oauth_client_id = match env::var("GOOGLE_OAUTH_CLIENT_ID"){
            Ok(google_oauth_client_id) => {
                google_oauth_client_id
            },
            Err(e) => {
                println!("{:?}", e);
                e.to_string()
            }
        };

        let google_oauth_client_secret = match env::var("GOOGLE_OAUTH_CLIENT_SECRET"){
            Ok(google_oauth_client_secret) => {
                google_oauth_client_secret
            },
            Err(e) => {
                println!("{:?}", e);
                e.to_string()
            }
        };

        let google_oauth_redirect_url = match env::var("GOOGLE_OAUTH_REDIRECT_URL"){
            Ok(google_oauth_redirect_url) => {
                google_oauth_redirect_url
            },
            Err(e) => {
                println!("{:?}", e);
                e.to_string()
            }
        };

         Arc::new(Config {
            google_oauth_client_id,
            google_oauth_client_secret,
            google_oauth_redirect_url
        })
    }
}

#[derive(Deserialize)]
pub struct OAuthResponse {
    pub access_token: String,
    pub id_token: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct GoogleUserResult {
    pub id: Option<String>,
    pub email: Option<String>,
    pub verified_email: bool,
    pub name: Option<String>,
    pub picture: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub locale: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthCodePayload {
    pub code: String,
}

/// Solicita um token ao Google OAuth2.
pub async fn request_token(
    config: &Config,
    authorization_code: &str,
) -> Result<OAuthResponse, (StatusCode, Json<String>)> {
    let client = Client::new();

    let params = [
        ("grant_type", "authorization_code"),
        ("redirect_uri", &config.google_oauth_redirect_url),
        ("client_id", &config.google_oauth_client_id),
        ("client_secret", &config.google_oauth_client_secret),
        ("code", authorization_code),
    ];

    let response = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(e.to_string())))?;

    if response.status().is_success() {
        response
            .json::<OAuthResponse>()
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, Json(format!("Failed to parse token response: {}", e))))
    } else {
        let error_message = response.text().await.unwrap_or_default();
        Err((StatusCode::BAD_REQUEST, Json(error_message)))
    }
}

/// Obtém informações do usuário usando o token.
pub async fn get_google_user(
    access_token: &str,
) -> Result<Json<GoogleUserResult>, (StatusCode, Json<String>)> {
    let client = Client::new();
    let mut url = Url::parse("https://www.googleapis.com/oauth2/v1/userinfo").unwrap();
    url.query_pairs_mut()
        .append_pair("access_token", access_token);

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, Json(e.to_string())))?;

    if !response.status().is_success() {
        let error_message = response.text().await.unwrap_or_default();
        return Err((StatusCode::BAD_REQUEST, Json(error_message)))
    }
    let response = response
            .json()
            .await;
            
    let res: GoogleUserResult = response.unwrap();
    return Ok(Json(res))
}

/// Handler para o endpoint de autenticação.
pub async fn google_oauth_handler(
    State(config): State<Arc<Config>>,
    Json(payload): Json<AuthCodePayload>,
) -> Result<Json<GoogleUserResult>, (StatusCode, Json<String>)> {
    // Solicita o token ao Google OAuth2
    let tokens = request_token(&config, &payload.code).await?;

    // Obtém informações do usuário
    let user_info = get_google_user(&tokens.access_token).await?;

    let fotoid = user_info.picture.clone().unwrap();
    println!("{}", fotoid);
    
    let res = cadastra_usuario_oauth(CredenciaisUsuarioGoogle{
        email: user_info.email.clone(),
        name: user_info.name.clone()
    }).await?;

    Ok(Json(user_info.0))
}

pub struct CredenciaisUsuarioGoogle{
    pub email: Option<String>,
    pub name: Option<String>
}

pub async fn cadastra_usuario_oauth(usuario: CredenciaisUsuarioGoogle)
    -> Result<(StatusCode, Json<UserId>), (StatusCode, Json<String>)>{
    match valida_usuario_oauth(&usuario).await{
        Ok(_) => {},
        Err(e) => {
            println!("{:?}", e);
            return Err((StatusCode::BAD_REQUEST, Json(e)))
        }
    }

    let email_clone = usuario.email.clone().unwrap();
    let nome_clone = usuario.name.clone().unwrap_or(email_clone.clone());
    let senha = gera_hash(&email_clone);

    let erro = match busca_usuario_email_oauth(Query(EmailInput{email: email_clone.clone()})).await{
        Ok(id) => {
            return Ok((StatusCode::OK, Json(UserId{idusuario: id.1.to_string()})))
        },
        Err(e) => {
            e
        }
    };

    if erro.1.0 == "Esse e-mail pertence a outro usuário.".to_string(){
        return Err(erro)
    }

    let idusuario = gera_hash(&email_clone);
    let idusuario_clone = idusuario.clone();
    let now = chrono::Utc::now().naive_utc();
    let usuario = Usuario{
        nome: nome_clone,
        email: email_clone.clone(),
        senha,
        documento: "Não definido".to_string(),
        datacadastro: now,
        idusuario,
        origemconta: Some("Google".to_string())
    };

    let conn = &mut cria_conn()?;

    match models::usuarios::cadastra_usuario(conn, usuario).await{
        Ok(_) => {
            
        },
        Err(e) => {
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(e)))
        }
    }

    let codigo = gera_codigo_recuperacao(email_clone.clone()).await?.1.0.codigo;
    match envia_email_codigo(email_clone, "ativação de conta", codigo).await{
        Ok(codigoativacao) => {
            return Ok((StatusCode::OK, Json(UserId{idusuario: idusuario_clone})))
        },
        Err(e) => {
            return Err(e)
        }
    }

}

pub async fn valida_usuario_oauth(usuario: &CredenciaisUsuarioGoogle) -> Result<(), String>{
    let nome = usuario.name.clone().unwrap_or("Usuário".to_string());
    if nome.trim().is_empty(){
        return Err("Erro ao validar o nome.".to_string())
    }

    let email = usuario.email.clone().unwrap();
    match valida_email(Json(EmailInput{email: email.clone()})).await{
        Ok(_) => {},
        Err(e) => {
            return Err(e.1.0)
        }
    }
    return Ok(())
}
