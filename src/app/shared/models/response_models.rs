use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum AppResponseStatus {
    Success,
    Error
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppResponse {
    pub status: AppResponseStatus,
    pub status_code: u16,
    pub message: String,
}
