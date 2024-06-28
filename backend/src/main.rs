use std::env;

use axum::Router;
use dotenvy::dotenv;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use crate::routers::upload_router;

mod controllers;
mod routers;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    let port = env::var("TOKEN").unwrap_or(String::from("3000"));
    let app = Router::new()
        .nest("/api/v1", upload_router())
        .nest_service("/uploads", ServeDir::new("../uploads"))
        .layer(CorsLayer::permissive());

    if let Ok(listener) = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await {
        println!("🚀 Server Running http://localhost:{}", port);
        axum::serve(listener, app).await.unwrap();
    }
}
