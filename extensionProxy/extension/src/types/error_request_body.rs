use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ErrorRequestBody {
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    #[serde(rename = "errorType")]
    pub error_type: String,
    #[serde(rename = "stackTrace")]
    pub stack_trace: Vec<String>,
}
