mod client;
mod endpoints;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactApiClient {
    #[serde(rename = "clientID")]
    pub client_id: String,
    #[serde(rename = "developerAPIKey")]
    pub developer_api_key: String,
    pub sandbox: bool,
}
