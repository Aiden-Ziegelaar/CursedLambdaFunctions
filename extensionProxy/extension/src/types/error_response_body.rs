use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ErrorResponseBody {
    #[serde(rename = "statusResponse")]
    pub status_response: Option<String>,
}
