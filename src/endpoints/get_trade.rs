use serde::{Deserialize, Serialize};

use crate::TransactApiClient;

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTradePayload {
    #[serde(rename = "clientID")]
    client_id: String,
    #[serde(rename = "developerAPIKey")]
    developer_api_key: String,
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "tradeId")]
    trade_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PartyDetail {
    id: String,
    #[serde(rename = "developerAPIKey")]
    developer_api_key: String,
    #[serde(rename = "offeringId")]
    offering_id: String,
    #[serde(rename = "accountId")]
    account_id: String,
    #[serde(rename = "partyId")]
    party_id: String,
    party_type: String,
    #[serde(rename = "escrowId")]
    escrow_id: String,
    #[serde(rename = "orderId")]
    order_id: String,
    #[serde(rename = "transactionType")]
    transaction_type: String,
    #[serde(rename = "totalAmount")]
    total_amount: String,
    #[serde(rename = "totalShares")]
    total_shares: String,
    #[serde(rename = "orderStatus")]
    order_status: String,
    #[serde(rename = "createdDate")]
    created_date: String,
    #[serde(rename = "createdIpAddress")]
    created_ip_address: String,
    #[serde(rename = "errors")]
    errors: String,
    #[serde(rename = "documentKey")]
    document_key: String,
    #[serde(rename = "esignStatus")]
    esign_status: String,
    users: String,
    archived_status: String,
    #[serde(rename = "RRApprovalStatus")]
    rr_approval_status: String,
    #[serde(rename = "RRName")]
    rr_name: String,
    #[serde(rename = "RRApprovalDate")]
    rr_approval_date: String,
    #[serde(rename = "PrincipalApprovalStatus")]
    principal_approval_status: String,
    #[serde(rename = "PrincipalName")]
    principal_name: String,
    #[serde(rename = "PrincipalDate")]
    principal_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTradeResponse {
    #[serde(rename = "statusCode")]
    pub status_code: String,
    #[serde(rename = "statusDesc")]
    pub status_description: String,
    #[serde(rename = "partyDetails")]
    pub party_details: Vec<PartyDetail>,
}

impl TransactApiClient {
    /// # Get Trade
    ///
    /// This method is used to get all the details of all the trades for an account. The Account ID
    /// is required to get the details.
    ///
    /// Reference: https://api.norcapsecurities.com/admin_v3/documentation?mid=MjEw
    ///
    /// ## Arguments:
    ///
    /// - `account_id` - Account ID generated by the API
    /// - `trade_id` - Trade ID generated by the API
    pub async fn get_trade(
        &self,
        account_id: String,
        trade_id: String,
    ) -> Result<GetTradeResponse, reqwest::Error> {
        let payload = GetTradePayload {
            client_id: self.client_id.to_owned(),
            developer_api_key: self.developer_api_key.to_owned(),
            account_id,
            trade_id,
        };

        let resp = TransactApiClient::post_request::<GetTradePayload, GetTradeResponse>(
            self,
            String::from("getTrade"),
            &payload,
        )
        .await?;

        Ok(resp)
    }
}
