use axum::{
    routing::{delete, get, post, put},
    Router,
    http::Method
};
use tower_http::cors::{Any, CorsLayer};

use crate::controllers::usuarios::estrutura_usuario;

pub fn cria_rotas() -> Router<>{
    let app: Router<_> = Router::new()
       // .route("/verifica_token", post(verifica_token))
       .route("/estrutura_usuario", post(estrutura_usuario))


        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::POST, Method::PUT]) 
                .allow_headers(Any),
        );
    app
}