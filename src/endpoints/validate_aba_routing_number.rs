use serde::{Deserialize, Serialize};

use crate::TransactApiClient;

#[derive(Debug, Serialize, Deserialize)]
struct ValidateAbaRoutingNumberPayload {
    #[serde(rename = "clientID")]
    client_id: String,
    #[serde(rename = "developerAPIKey")]
    developer_api_key: String,
    #[serde(rename = "routingNumber")]
    routing_number: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidateAbaRoutingNumberResponse {
    #[serde(rename = "statusCode")]
    pub status_code: String,
    #[serde(rename = "statusDesc")]
    pub status_description: String,
    #[serde(rename = "accountDetails")]
    pub account_details: String,
}

impl TransactApiClient {
    /// This method is used to validate the routing number for an external account
    /// (via `createExternalAccount`).
    ///
    /// Reference: https://api.norcapsecurities.com/admin_v3/documentation?mid=MjU1
    ///
    /// # Arguments
    ///
    /// - `routing_number` - The routing number to be validated
    pub async fn validate_aba_routing_number(
        &self,
        routing_number: String,
    ) -> Result<ValidateAbaRoutingNumberResponse, reqwest::Error> {
        let url = TransactApiClient::base_url(&self).to_owned() + "validateABARoutingNumber";
        let get_offering_payload = ValidateAbaRoutingNumberPayload {
            client_id: self.client_id.to_owned(),
            developer_api_key: self.developer_api_key.to_owned(),
            routing_number,
        };
        let client = reqwest::Client::new();
        let res = client
            .post(url)
            .json(&get_offering_payload)
            .send()
            .await?
            .json::<ValidateAbaRoutingNumberResponse>()
            .await?;
        Ok(res)
    }
}
