use axum::{response::Json, routing::get, Router, Server};
use serde_json::json;
use std::net::SocketAddr;

async fn health() -> Json<serde_json::Value> {
    Json(json!({
        "status": "ok",
        "service": "charge-mgt-cloud",
        "version": "0.1.0"
    }))
}

async fn root() -> Json<serde_json::Value> {
    Json(json!({
        "name": "charge-mgt-cloud",
        "description": "OCPP Charging Station Management System"
    }))
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health));

    println!("🚀 charge-mgt-cloud starting on http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
