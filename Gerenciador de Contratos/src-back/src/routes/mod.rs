use axum::{
    http::Method, routing::{get, patch, post}, Router
};
use tower_http::cors::{Any, CorsLayer};

use crate::controllers::{codigos_recuperacao::{envia_codigo_recuperacao, verifica_codigo_recuperacao}, maquinas::{cadastra_maquina, lista_todas_maquinas}, usuarios::{atualiza_email_usuario, atualiza_senha_usuario, busca_email_usuario, busca_senha_usuario, cadastra_usuario, realiza_login}};
use crate::controllers::usuarios::busca_usuario_email;

pub fn cria_rotas() -> Router<>{
    let app: Router<_> = Router::new()
        .route("/cadastra_usuario", post(cadastra_usuario))
        .route("/busca_email_usuario", get(busca_email_usuario))
        .route("/realiza_login", post(realiza_login))
        .route("/busca_senha_usuario", post(busca_senha_usuario))
        .route("/atualiza_senha_usuario", patch(atualiza_senha_usuario))
        .route("/atualiza_email_usuario", patch(atualiza_email_usuario))
        .route("/busca_usuario_email", get(busca_usuario_email))

        .route("/verifica_codigo_recuperacao", post(verifica_codigo_recuperacao))
        .route("/envia_codigo_recuperacao", post(envia_codigo_recuperacao))

        .route("/cadastra_maquina", post(cadastra_maquina))
        .route("/lista_todas_maquinas", get(lista_todas_maquinas))

        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::POST, Method::PUT, Method::PATCH]) 
                .allow_headers(Any),
        );
    app
}