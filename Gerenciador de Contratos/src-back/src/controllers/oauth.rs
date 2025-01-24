use axum::{extract::State, Json};
use hyper::StatusCode;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub google_oauth_client_id: String,
    pub google_oauth_client_secret: String,
    pub google_oauth_redirect_url: String,
}

impl Config {
    pub fn init() -> Arc<Self> {
        let google_oauth_client_id = "/".to_string();
        let google_oauth_client_secret = "/".to_string();
        let google_oauth_redirect_url = "/".to_string();
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

#[derive(Deserialize, Serialize)]
pub struct GoogleUserResult {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub picture: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthCodePayload {
    pub code: String,
}

/// Solicita um token ao Google OAuth2.
pub async fn request_token(
    config: &Config,
    authorization_code: &str,
) -> Result<OAuthResponse, (StatusCode, String)> {
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
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    if response.status().is_success() {
        response
            .json::<OAuthResponse>()
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to parse token response: {}", e)))
    } else {
        let error_message = response.text().await.unwrap_or_default();
        Err((StatusCode::BAD_REQUEST, error_message))
    }
}

/// Obtém informações do usuário usando o token.
pub async fn get_google_user(
    access_token: &str,
) -> Result<Json<GoogleUserResult>, (StatusCode, String)> {
    let client = Client::new();
    let mut url = Url::parse("https://www.googleapis.com/oauth2/v1/userinfo").unwrap();
    url.query_pairs_mut()
        .append_pair("access_token", access_token);

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    if !response.status().is_success() {
        let error_message = response.text().await.unwrap_or_default();
        return Err((StatusCode::BAD_REQUEST, error_message))
    }
    println!("Res get_google_user: {:?}", response);
    let response = response
            .json()
            .await
            .map_err(|e| (StatusCode::BAD_REQUEST, format!("Failed to parse user info: {}", e)));
    let res = response.unwrap();
    return Ok(Json(res))
}

/// Handler para o endpoint de autenticação.
pub async fn google_oauth_handler(
    State(config): State<Arc<Config>>,
    Json(payload): Json<AuthCodePayload>,
) -> Result<Json<GoogleUserResult>, (StatusCode, String)> {
    // Solicita o token ao Google OAuth2
    println!("Code: {:?}", &payload.code);
    let tokens = request_token(&config, &payload.code).await?;

    // Obtém informações do usuário
    let user_info = get_google_user(&tokens.access_token).await?;

    Ok(Json(user_info.0))
}
