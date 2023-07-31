use axum::response::Json;
use serde_json::{json, Value};

pub async fn do_something() -> Json<Value> {
    println!("do something");
    Json(json!({ "msg": "I am GET /" }))
}
