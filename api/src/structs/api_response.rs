use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse {
    pub tx_hash: String,
}