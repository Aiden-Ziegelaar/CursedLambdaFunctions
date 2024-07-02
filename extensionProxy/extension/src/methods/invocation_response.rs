use axum::{
    extract::Path, http::{HeaderMap, StatusCode}, Json
};

use crate::{LAMBDA_RUNTIME_API_VERSION, PROXY_PORT};

pub async fn post_invocation_response(
        headers: axum::http::HeaderMap,
        Path(invocation_id): Path<String>,
        Json(payload): Json<serde_json::Value>,
) -> (StatusCode, HeaderMap, Json<serde_json::Value>) {

    let client = reqwest::Client::new();

    let api_response = client.post(format!("http://localhost:{}/{}/invocation/{}/error",PROXY_PORT, LAMBDA_RUNTIME_API_VERSION, invocation_id))
        .body(serde_json::to_string(&payload).unwrap())
        .headers(headers.clone())
        .send()
        .await
        .unwrap();

    let status = api_response.status();
    let response_headers = api_response.headers().clone();
    let body = api_response.json::<serde_json::Value>().await.unwrap();

    (status, response_headers, Json(body))
}
