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
    let grpc_server = Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr);

    let grpc_task = tokio::spawn(async move {
        if let Err(e) = grpc_server.await {
            eprintln!("Erro no servidor gRPC: {}", e);
        }
    });

    let axum_task = tokio::spawn(async move {
        if let Err(e) = axum::serve(listener, app).await {
            eprintln!("Erro no servidor HTTP: {}", e);
        }
    });

    tokio::try_join!(grpc_task, axum_task).unwrap();
}
