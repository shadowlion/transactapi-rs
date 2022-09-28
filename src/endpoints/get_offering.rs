use serde::{Deserialize, Serialize};

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

pub async fn get_offering(
    payload: GetOfferingPayload,
) -> Result<GetOfferingResponse, reqwest::Error> {
    let url = "";
    let client = reqwest::Client::new();
    let resp: GetOfferingResponse = client.post(url).json(&payload).send().await?.json().await?;
    Ok(resp)
}
