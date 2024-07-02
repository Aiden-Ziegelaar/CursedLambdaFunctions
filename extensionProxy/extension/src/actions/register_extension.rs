use axum::http::HeaderMap;
use lambda_extension::tracing;

const EXTENSION_API_VERSION: &str = "2020-01-01";

pub async fn register_extension() {

    let client = reqwest::Client::new();

    let mut headers = HeaderMap::new();

    headers.insert("Lambda-Extension-Name", crate::EXTENSION_NAME.parse().unwrap());

    let api_response = client.post(format!("http://{}/{}/extension/register", crate::env::env::sandbox_runtime_api(), EXTENSION_API_VERSION))
        .body(r#"{"events":["SHUTDOWN"]}"#)
        .headers(headers)
        .send()
        .await
        .unwrap();

    match api_response.status() {
        reqwest::StatusCode::OK => {
            tracing::info!("Extension registered successfully");
        }
        _ => {
            tracing::error!("Failed to register extension: {:?}", api_response.text().await);
        }
    }
}