use axum::{
    routing::{delete, get, post, put},
    Router,
    http::Method
};
use tower_http::cors::{Any, CorsLayer};


use crate::controller::{
    checa_email, compara_novas_senhas, encontra_email_smtp, endereco::{
        _salva_endereco, busca_endereco_id
    }, gera_token, maquina::estrutura_maquina, usuario::{
        atualiza_email, atualiza_nome, atualiza_senha, busca_cnpj_usuario, busca_email_usuario, busca_id, busca_nome_usuario, cria_conta, deleta_conta, verifica_senha, verifica_token
    }, verifica_codigo_email
};


pub fn cria_rotas() -> Router<>{
    let app: Router<_> = Router::new()
        .route("/_salva_endereco", post(_salva_endereco))
        .route("/verifica_senha", post(verifica_senha))
        .route("/cria_conta", post(cria_conta))
        .route("/busca_endereco_id", get(busca_endereco_id))
        .route("/atualiza_email", put(atualiza_email))
        .route("/atualiza_senha", put(atualiza_senha))
        .route("/verifica_token", post(verifica_token))
        .route("/busca_id", post(busca_id))
        .route("/atualiza_nome", put(atualiza_nome))
        .route("/busca_email_usuario", get(busca_email_usuario))
        .route("/busca_nome_usuario", get(busca_nome_usuario))
        .route("/busca_cnpj_usuario", get(busca_cnpj_usuario))
        .route("/deleta_conta", delete(deleta_conta))
        .route("/checa_email", post(checa_email))
        .route("/gera_token", post(gera_token))
        .route("/encontra_email_smtp", post(encontra_email_smtp))
        .route("/verifica_codigo_email", post(verifica_codigo_email))
        .route("/compara_novas_senhas", post(compara_novas_senhas))

        .route("/estrutura_maquina", post(estrutura_maquina))

        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(vec![Method::POST, Method::PUT]) 
                .allow_headers(Any),
        );
    app
}