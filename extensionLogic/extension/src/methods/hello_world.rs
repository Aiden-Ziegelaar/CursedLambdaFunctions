use axum::{
    http::StatusCode,
    Json,
};

use serde_json::json;

pub async fn hello_world(
    Json(body): Json<serde_json::Value>
) -> (StatusCode, Json<serde_json::Value>) {

    let response = match body.get("name") {
        Some(name) => json!({"message": format!("Hello, {}!", name.as_str().unwrap())}),
        None => json!({"message": "Hello, World!"})
    };
    
    (StatusCode::OK, Json(response))
}
