use std::env;

pub mod controllers;
pub mod models;
pub mod routes;
pub mod schema;
pub mod tests;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();
    let db_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Erro: {}", e);
            "".to_string()
        }
    };
    let app = crate::routes::cria_rotas(&db_url);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
