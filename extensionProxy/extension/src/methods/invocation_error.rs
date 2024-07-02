use axum::{
    body::Bytes, extract::Path, http::{HeaderMap, StatusCode}
};
use lambda_extension::tracing;

use crate::LAMBDA_RUNTIME_API_VERSION;

pub async fn post_invocation_error(
        headers: axum::http::HeaderMap,
        Path(invocation_id): Path<String>,
        payload: Bytes
    ) -> (StatusCode, HeaderMap, Bytes) {

    tracing::info!(event_type = "errorEvent", event = ?payload, "invoking");

    let client = reqwest::Client::new();

    let api_response = client.post(format!("http://{}/{}/runtime/invocation/{}/error", crate::env::env::sandbox_runtime_api(), LAMBDA_RUNTIME_API_VERSION, invocation_id))
        .body(payload)
        .headers(headers.clone())
        .send()
        .await
        .unwrap();

    let status = api_response.status();
    let response_headers = api_response.headers().clone();
    let body = api_response.bytes().await.unwrap();

    (status, response_headers, body)
}
