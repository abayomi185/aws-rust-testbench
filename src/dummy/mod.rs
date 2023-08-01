use axum::{extract::Path, response::Json};
use serde_json::{json, Value};
use tracing::info;

pub async fn root() -> Json<Value> {
    info!("root");
    Json(json!({ "msg": "I am GET /" }))
}

pub async fn get_foo() -> Json<Value> {
    info!("get_foo");
    Json(json!({ "msg": "I am GET /foo" }))
}

pub async fn post_foo() -> Json<Value> {
    Json(json!({ "msg": "I am POST /foo" }))
}

pub async fn post_foo_name(Path(name): Path<String>) -> Json<Value> {
    info!("get_foo");
    Json(json!({ "msg": format!("I am POST /foo/:name, name={name}") }))
}
