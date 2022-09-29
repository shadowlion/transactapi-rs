use serde::{de, Serialize};

use crate::TransactApiClient;

impl TransactApiClient {
    /// Instantiates  newa TransactApi client
    ///
    /// # Arguments
    ///
    /// - `client_id` - client identifier
    /// - `developer_api_key` - developer api key
    /// - `sandbox` - flag for using sandbox api base url
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
    pub fn base_url(&self) -> String {
        let mut prefix: &str = "api";
        if self.sandbox {
            prefix = "api-sandboxdash";
        }
        format!(
            "https://{}.norcapsecurities.com/tapiv3/index.php/v3/",
            prefix
        )
    }

    /// Use reqwest to run a post-method api call.
    ///
    /// # Arguments
    ///
    /// - `&self` - TransactApiClient instance
    /// - `endpoint` - string endpoint
    /// - `payload` - data transfer struct (generic)
    pub async fn post_request<Request: Serialize, Response: de::DeserializeOwned>(
        &self,
        endpoint: String,
        payload: &Request,
    ) -> Result<Response, reqwest::Error> {
        let url = TransactApiClient::base_url(&self).to_owned() + &endpoint;
        let client = reqwest::Client::new();
        let res = client
            .post(url)
            .json(&payload)
            .send()
            .await?
            .json::<Response>()
            .await?;
        Ok(res)
    }
}
