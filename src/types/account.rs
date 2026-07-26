use serde::{Deserialize, Serialize};

/// A Chimoney transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub id: String,
    pub amount: f64,
    pub currency: String,
    pub status: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Request to transfer between accounts.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRequest {
    /// The receiver's ID or email.
    pub receiver: String,
    /// Amount in USD.
    pub value_in_usd: f64,
    /// Optional sub-account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
}

/// Response from a transfer request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferResponse {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
}

/// Request to initiate a Chimoney transaction.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InitiateChimoneyRequest {
    /// Receiver's email or ID.
    pub receiver: String,
    /// Amount in USD.
    pub value_in_usd: f64,
    /// Optional sub-account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
    /// Optional turn off notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_off_notification: Option<bool>,
}

/// Response from initiating a Chimoney transaction.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitiateChimoneyResponse {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
}

/// Request to delete unpaid transactions.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUnpaidTransactionRequest {
    /// The chi reference to delete.
    pub chi_ref: String,
}

/// Response from deleting unpaid transactions.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteUnpaidTransactionResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
}

/// Request to issue an Interledger wallet address.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueWalletAddressRequest {
    /// User ID.
    #[serde(rename = "userID")]
    pub user_id: String,
    /// Interledger username.
    pub ilp_username: String,
}

/// Request to claim a community membership reward.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimRewardRequest {
    /// User ID claiming the reward.
    pub user_id: String,
    /// Community details.
    pub community: ClaimRewardCommunity,
}

/// Community details for reward claim.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimRewardCommunity {
    /// Community ID.
    #[serde(rename = "communityID")]
    pub community_id: String,
    /// Membership ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_id: Option<String>,
    /// Voucher code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher_code: Option<String>,
    /// Community name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Member name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_name: Option<String>,
    /// Community type ("free" or "paid").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub community_type: Option<String>,
    /// Expiry date.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
}

/// Response from account operations.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountOperationResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

/// A single issue ID transaction item.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueIdTransactionItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub account_number: Option<String>,
    #[serde(default)]
    pub issue_id: Option<String>,
    #[serde(default)]
    pub fee: Option<f64>,
    #[serde(default)]
    pub account_bank: Option<String>,
    #[serde(default)]
    pub r#type: Option<String>,
    #[serde(default)]
    pub issuer: Option<String>,
    #[serde(default)]
    pub enabled_to_redeem: Option<Vec<String>>,
    #[serde(default)]
    pub reference: Option<String>,
    #[serde(default)]
    pub chi_ref: Option<String>,
    #[serde(default)]
    pub issue_date: Option<String>,
    #[serde(default)]
    pub initiated_by: Option<String>,
    #[serde(default)]
    pub redeem_data: Option<serde_json::Value>,
    #[serde(default)]
    pub value_in_usd: Option<f64>,
    #[serde(default)]
    pub chimoney: Option<f64>,
    #[serde(default)]
    pub country_to_send: Option<String>,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub personalized_message: Option<String>,
    #[serde(default)]
    pub branch_code: Option<String>,
    #[serde(default)]
    pub t_id: Option<i64>,
    #[serde(default)]
    pub collection_payment_issue_id: Option<String>,
    #[serde(default)]
    pub narration: Option<String>,
    #[serde(default)]
    pub fullname: Option<String>,
    #[serde(default)]
    pub payment_date: Option<String>,
    #[serde(default)]
    pub redeem_date: Option<String>,
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
    #[serde(default)]
    pub payout: Option<serde_json::Value>,
    #[serde(default)]
    pub updated_date: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub delivery_status: Option<String>,
}

/// Public profile verification info.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileVerification {
    #[serde(default)]
    pub status: Option<String>,
}

/// Public profile payment data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfilePaymentData {
    #[serde(default)]
    pub interledger_wallet_address: Option<String>,
}

/// Public profile of a Chimoney user.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicProfile {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub account_second_currencies: Option<Vec<String>>,
    #[serde(default)]
    pub preferred_exchange_rate: Option<bool>,
    #[serde(default)]
    pub created_date: Option<String>,
    #[serde(default)]
    pub join_date: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub verification: Option<ProfileVerification>,
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub career: Option<String>,
    #[serde(default)]
    pub purpose_of_chimoney: Option<String>,
    #[serde(default)]
    pub socials: Option<serde_json::Value>,
    #[serde(default)]
    pub updated_date: Option<String>,
    #[serde(default)]
    pub p_id: Option<i64>,
    #[serde(default)]
    pub points: Option<i64>,
    #[serde(default)]
    pub badges: Option<Vec<String>>,
    #[serde(default)]
    pub payment_data: Option<ProfilePaymentData>,
}
