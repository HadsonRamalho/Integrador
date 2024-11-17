use axum::{
    routing::{get, post},
    Router,
    http::Method
};
use tower_http::cors::{Any, CorsLayer};

use crate::controllers::usuarios::{busca_email_usuario, busca_senha_usuario, cadastra_usuario, estrutura_usuario, realiza_login};

pub fn cria_rotas() -> Router<>{
    let app: Router<_> = Router::new()
        .route("/estrutura_usuario", post(estrutura_usuario))
        .route("/cadastra_usuario", post(cadastra_usuario))
        .route("/busca_email_usuario", get(busca_email_usuario))
        .route("/realiza_login", post(realiza_login))
        .route("/busca_senha_usuario", post(busca_senha_usuario))

        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::POST, Method::PUT]) 
                .allow_headers(Any),
        );
    app
}