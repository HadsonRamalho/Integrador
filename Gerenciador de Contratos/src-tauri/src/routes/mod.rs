use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::controller::{
    endereco::{
        _salva_endereco, busca_endereco_id
    }, 
    usuario::{
        atualiza_email, atualiza_nome, atualiza_senha, busca_id, cria_conta, verifica_senha, verifica_token
    }
};

pub fn cria_rotas() -> Router<>{
    let app: Router<_> = Router::new()
        .route("/_salva_endereco", post(_salva_endereco))
        .route("/verifica_senha", post(verifica_senha))
        .route("/cria_conta", post(cria_conta))
        .route("/busca_endereco_id", get(busca_endereco_id))
        .route("/atualiza_email", put(atualiza_email))
        .route("/atualiza_senha", put(atualiza_senha))
        .route("/verifica_token", get(verifica_token))
        .route("/busca_id", get(busca_id))
        .route("/atualiza_nome", put(atualiza_nome));
    app
}