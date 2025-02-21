pub mod controllers;
pub mod models;
pub mod tests;
pub mod routes;
pub mod schema;
pub mod server;
pub mod client;
use tonic::transport::Server;
pub mod mid;
use crate::server::hello_world::greeter_server::GreeterServer;
use crate::server::MyGreeter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = crate::routes::cria_rotas();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await.unwrap();
    axum::serve(listener, app).await.unwrap();
}