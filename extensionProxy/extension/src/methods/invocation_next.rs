use axum::{
    http::{HeaderMap, StatusCode}, Json
};
use lambda_extension::tracing;
use serde_json::json;

use crate::LAMBDA_RUNTIME_API_VERSION;

pub async fn get_invocation_next(
        headers: axum::http::HeaderMap,
) -> (StatusCode, HeaderMap, Json<serde_json::Value>) {

    let client = reqwest::Client::new();

    let api_response = client.get(format!("http://{}/{}/runtime/invocation/next", crate::env::env::sandbox_runtime_api(), LAMBDA_RUNTIME_API_VERSION))
        .headers(headers.clone())
        .send()
        .await
        .unwrap();

    let status = api_response.status();
    let response_headers = api_response.headers().clone();
    
    let mut body = api_response.json::<serde_json::Value>().await.unwrap();

    tracing::info!(event_type = "requestEvent", event = ?body, "invoking");

    match body.get_mut("name") {
        Some(name) => {
            *name = json!(format!("{} and rustProxy", name.as_str().unwrap()))
        }
        None => {}
    }

    tracing::info!(event_type = "requestEvent", event = ?body, "invoking");

    (status, response_headers, Json(body))
}
