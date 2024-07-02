use axum::{
    http::{HeaderMap, StatusCode}, Json
};

use crate::{LAMBDA_RUNTIME_API_VERSION, PROXY_PORT};

pub async fn get_invocation_next(
        headers: axum::http::HeaderMap,
) -> (StatusCode, HeaderMap, Json<serde_json::Value>) {

    let client = reqwest::Client::new();

    let api_response = client.get(format!("http://localhost:{}/{}/invocation/next",PROXY_PORT, LAMBDA_RUNTIME_API_VERSION))
        .headers(headers.clone())
        .send()
        .await
        .unwrap();

    
    let status = api_response.status();
    let response_headers = api_response.headers().clone();
    let mut body = api_response.json::<serde_json::Value>().await.unwrap();

    match body.get_mut("name") {
        Some(name) => {
            *name = serde_json::Value::String("Rustacean".to_string());
        }
        None => {}
    }

    (status, response_headers, Json(body))
}
