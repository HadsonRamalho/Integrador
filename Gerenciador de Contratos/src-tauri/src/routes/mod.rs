use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::controller::{
    endereco::{_salva_endereco, busca_endereco_id, estrutura_endereco}, usuario::{cria_conta, verifica_senha}
};

pub fn cria_rotas() -> Router<>{
    let app: Router<_> = Router::new()
        .route("/_salva_endereco", post(_salva_endereco))
        .route("/verifica_senha", post(verifica_senha))
        .route("/cria_conta", post(cria_conta))
        .route("/busca_endereco_id", get(busca_endereco_id));
    app
}