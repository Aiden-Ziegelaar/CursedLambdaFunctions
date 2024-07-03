use axum::{
    http::{HeaderMap, StatusCode},
    Json,
};

use crate::{
    types::{error_request_body::ErrorRequestBody, error_response_body::ErrorResponseBody},
    LAMBDA_RUNTIME_API_VERSION,
};

pub async fn post_init_error(
    headers: axum::http::HeaderMap,
    Json(payload): Json<ErrorRequestBody>,
) -> (StatusCode, HeaderMap, Json<ErrorResponseBody>) {
    let client = reqwest::Client::new();

    let api_response = client
        .post(format!(
            "http://{}/{}/init/error",
            crate::env::env::sandbox_runtime_api(),
            LAMBDA_RUNTIME_API_VERSION
        ))
        .body(serde_json::to_string(&payload).unwrap())
        .headers(headers.clone())
        .send()
        .await
        .unwrap();

    let status = api_response.status();
    let response_headers = api_response.headers().clone();
    let body = api_response.json::<ErrorResponseBody>().await.unwrap();

    (status, response_headers, Json(body))
}
