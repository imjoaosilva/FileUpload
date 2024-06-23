use std::env;

use axum::Router;
use dotenvy::dotenv;

mod controllers;
mod routers;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let port = env::var("TOKEN").unwrap_or(String::from("3000"));
    let app = Router::new();
    if let Ok(listener) = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await {
        println!("🚀 Server Running http://localhost:{}", port);
        axum::serve(listener, app).await.unwrap();
    }
}
