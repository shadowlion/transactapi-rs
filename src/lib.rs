use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactApiClient {
    #[serde(rename = "clientID")]
    pub client_id: String,
    #[serde(rename = "developerAPIKey")]
    pub developer_api_key: String,
    pub sandbox: bool,
}

mod endpoints;

impl TransactApiClient {
    pub fn new(client_id: String, developer_api_key: String, sandbox: bool) -> TransactApiClient {
        TransactApiClient {
            client_id,
            developer_api_key,
            sandbox,
        }
    }

    /// Returns a url that will access the TransactApi server.
    ///
    /// # Arguments
    ///
    /// - `&self` - struct instance of the TransactApi client. Houses `sandbox` to determine if
    /// we're using sandbox credentials.
    fn base_url(&self) -> String {
        let mut prefix: &str = "api";
        if self.sandbox {
            prefix = "api-sandboxdash";
        }
        format!(
            "https://{}.norcapsecurities.com/tapiv3/index.php/v3/",
            prefix
        )
    }
}
