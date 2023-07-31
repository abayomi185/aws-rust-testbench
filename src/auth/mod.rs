use axum::response::Json;
use serde_json::{json, Value};

pub async fn login() -> Json<Value> {
    println!("login");
    Json(json!({ "msg": "I am GET /" }))
}

pub async fn signup() -> Json<Value> {
    println!("signup");
    Json(json!({ "msg": "I am GET /" }))
}
