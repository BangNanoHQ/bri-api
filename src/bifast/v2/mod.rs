use reqwest::Client;
use serde::{Deserialize, Serialize};

pub struct BifastClient {
    client: Client,
    base_url: String,
    api_key: String,
}

impl BifastClient {
    pub fn new(base_url: String, api_key: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
            api_key,
        }
    }

    // Add your API methods here
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub status: String,
    pub data: Option<T>,
    pub message: Option<String>,
}
