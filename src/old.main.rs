#![forbid(unsafe_code)]

use std::{collections::HashMap, net::SocketAddr};

use aws_rust_testbench::foo;

use serde::{Deserialize, Serialize};
use tracing::info;

use axum::{
    extract::Query,
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use serde_json::{json, Value};

// async fn handler(event) -> Result<> {
//     Ok()
// }

/// .
///
/// # Panics
///
/// Panics if .
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json() // Logged messages come out as json
        .init();

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/foo", get(|| async { "Hello, World!" }))
        .route("/bar", post(json_handler));

    // run it with hyper on localhost:3000
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    // axum::Server::bind(&"0.0.0.0:4000".parse().unwrap())
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Responses implement IntoResponse
async fn json_handler(Query(params): Query<HashMap<String, String>>) -> Json<Value> {
    info!("Handling request");

    let query = params.get("k").unwrap_or(&"Nothing found".to_string());

    Json /* From Axum */(json!({"message": "Hello World"}))
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItem {
    pub title: String,
    pub notes: String,
}
