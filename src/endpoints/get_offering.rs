use serde::{Deserialize, Serialize};

use crate::TransactApiClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOfferingPayload {
    #[serde(rename = "clientID")]
    client_id: String,
    #[serde(rename = "developerAPIKey")]
    developer_api_key: String,
    #[serde(rename = "offeringId")]
    offering_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OfferingDetail {
    #[serde(rename = "offeringId")]
    pub offering_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetOfferingResponse {
    #[serde(rename = "statusCode")]
    pub status_code: String,
    #[serde(rename = "statusDesc")]
    pub status_description: String,
    #[serde(rename = "offeringDetails")]
    pub offering_details: Vec<OfferingDetail>,
}

impl TransactApiClient {
    /// This method is used to get all the details of an offering. The Offering ID is required to
    /// get the information.
    ///
    /// Reference: https://api.norcapsecurities.com/admin_v3/documentation?mid=MTY4
    ///
    /// # Arguments:
    ///
    /// - `offering_id` - Offering ID that is generated by the API when an Offering is created
    /// (createOffering).
    pub async fn get_offering(
        &self,
        offering_id: String,
    ) -> Result<GetOfferingResponse, reqwest::Error> {
        let url = TransactApiClient::base_url(&self).to_owned() + "getOffering";
        let get_offering_payload = GetOfferingPayload {
            client_id: self.client_id.to_owned(),
            developer_api_key: self.developer_api_key.to_owned(),
            offering_id,
        };
        let client = reqwest::Client::new();
        let res = client
            .post(url)
            .json(&get_offering_payload)
            .send()
            .await?
            .json::<GetOfferingResponse>()
            .await?;
        Ok(res)
    }
}
