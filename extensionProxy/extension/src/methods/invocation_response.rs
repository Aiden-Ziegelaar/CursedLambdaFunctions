use axum::{
    body::Bytes, extract::Path, http::{HeaderMap, StatusCode}
};

use crate::LAMBDA_RUNTIME_API_VERSION;

pub async fn post_invocation_response(
        Path(invocation_id): Path<String>,
        payload: Bytes,
) -> (StatusCode, HeaderMap, Bytes) {

    let client = reqwest::Client::new();

    lambda_extension::tracing::info!(event_type = "responseEvent", event = ?payload, "invoking");

    let api_response = client.post(format!("http://{}/{}/runtime/invocation/{}/response", crate::env::env::sandbox_runtime_api(), LAMBDA_RUNTIME_API_VERSION, invocation_id))
        .body(payload)
        .send()
        .await
        .unwrap();

    let status = api_response.status();
    let response_headers = api_response.headers().clone();
    let body = api_response.bytes().await.unwrap();

    lambda_extension::tracing::info!(event_type = "responseEvent", event = ?body, "invoking");

    (status, response_headers, body)
}
