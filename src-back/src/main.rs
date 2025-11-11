pub mod controllers;
pub mod models;
pub mod tests;
pub mod routes;
pub mod schema;

#[tokio::main]
 async fn main() {
     tracing_subscriber::fmt::init();
     let app = crate::routes::cria_rotas();
     let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
     axum::serve(listener, app).await.unwrap();
 }