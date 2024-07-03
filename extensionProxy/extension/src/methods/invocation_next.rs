use axum::{
    http::{HeaderMap, StatusCode},
    Json,
};

use serde_json::json;

use crate::{http_client, LAMBDA_RUNTIME_API_VERSION};

pub async fn get_invocation_next(
    headers: axum::http::HeaderMap,
) -> (StatusCode, HeaderMap, Json<serde_json::Value>) {
    let client = http_client::client();

    let api_response = client
        .get(format!(
            "http://{}/{}/runtime/invocation/next",
            crate::env::env::sandbox_runtime_api(),
            LAMBDA_RUNTIME_API_VERSION
        ))
        .headers(headers.clone())
        .send()
        .await
        .unwrap();

    let status = api_response.status();
    let mut response_headers = api_response.headers().clone();

    let mut body = api_response.json::<serde_json::Value>().await.unwrap();

    match body.get_mut("name") {
        Some(name) => {
            *name = json!(format!("{} and Rust", name.as_str().unwrap()));
        }
        None => {}
    }

    response_headers.remove("Content-Length"); // Lmbda runtime preallocates a buffer for the response, so we need to remove the content-length header from the original so axum can calculate the correct length

    (status, response_headers, Json(body))
}
