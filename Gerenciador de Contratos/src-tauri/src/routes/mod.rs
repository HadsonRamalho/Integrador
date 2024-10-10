use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::controller::endereco::{_salva_endereco, estrutura_endereco};

pub fn cria_rotas() -> Router<>{
    let app: Router<_> = Router::new()
        .route("/_salva_endereco", post(_salva_endereco));
    app
}