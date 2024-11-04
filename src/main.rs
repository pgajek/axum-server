use axum::{
    extract::{Path, Query},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/json", post(handle_json));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8123));
    println!("Listening on {:?}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Welcome to the Axum server!"
}

#[derive(Deserialize)]
struct InputData {
    message: String,
}

#[derive(Serialize)]
struct OutputData {
    response: String,
}

async fn handle_json(Json(payload): Json<InputData>) -> impl IntoResponse {
    let response = OutputData {
        response: format!("You sent: {}", payload.message),
    };
    Json(response)
}
