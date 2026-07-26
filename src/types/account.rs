use serde::{Deserialize, Serialize};

/// A Chimoney transaction.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    /// Unique transaction identifier.
    pub id: String,
    /// Transaction amount.
    pub amount: f64,
    /// ISO 4217 currency code.
    pub currency: String,
    /// Transaction status.
    pub status: String,
    /// Optional transaction description.
    #[serde(default)]
    pub description: Option<String>,
    /// ISO 8601 creation timestamp.
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
    /// Unique transfer identifier.
    pub id: String,
    /// Response status (`"success"` or `"error"`).
    pub status: String,
    /// Human-readable status message.
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
    /// Unique transaction identifier.
    pub id: String,
    /// Response status (`"success"` or `"error"`).
    pub status: String,
    /// Human-readable status message.
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
    /// Response status (`"success"` or `"error"`).
    pub status: String,
    /// Human-readable status message.
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
    /// Response status (`"success"` or `"error"`).
    pub status: String,
    /// Human-readable status message.
    #[serde(default)]
    pub message: Option<String>,
    /// Response data payload.
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

/// A single issue ID transaction item.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueIdTransactionItem {
    /// Unique transaction identifier.
    #[serde(default)]
    pub id: Option<String>,
    /// Bank account number.
    #[serde(default)]
    pub account_number: Option<String>,
    /// Issue ID.
    #[serde(default)]
    pub issue_id: Option<String>,
    /// Transaction fee.
    #[serde(default)]
    pub fee: Option<f64>,
    /// Bank name.
    #[serde(default)]
    pub account_bank: Option<String>,
    /// Transaction type.
    #[serde(default)]
    pub r#type: Option<String>,
    /// Issuer identifier.
    #[serde(default)]
    pub issuer: Option<String>,
    /// List of accounts enabled to redeem.
    #[serde(default)]
    pub enabled_to_redeem: Option<Vec<String>>,
    /// Transaction reference.
    #[serde(default)]
    pub reference: Option<String>,
    /// Chimoney reference.
    #[serde(default)]
    pub chi_ref: Option<String>,
    /// ISO 8601 issue date.
    #[serde(default)]
    pub issue_date: Option<String>,
    /// User who initiated the transaction.
    #[serde(default)]
    pub initiated_by: Option<String>,
    /// Redemption data payload.
    #[serde(default)]
    pub redeem_data: Option<serde_json::Value>,
    /// Transaction value in USD.
    #[serde(default)]
    pub value_in_usd: Option<f64>,
    /// Chimoney amount.
    #[serde(default)]
    pub chimoney: Option<f64>,
    /// Destination country code.
    #[serde(default)]
    pub country_to_send: Option<String>,
    /// Transaction amount in local currency.
    #[serde(default)]
    pub amount: Option<f64>,
    /// Personalized message attached to the transaction.
    #[serde(default)]
    pub personalized_message: Option<String>,
    /// Bank branch code.
    #[serde(default)]
    pub branch_code: Option<String>,
    /// Transaction ID.
    #[serde(default)]
    pub t_id: Option<i64>,
    /// Collection payment issue ID.
    #[serde(default)]
    pub collection_payment_issue_id: Option<String>,
    /// Transaction narration.
    #[serde(default)]
    pub narration: Option<String>,
    /// Full name of the recipient.
    #[serde(default)]
    pub fullname: Option<String>,
    /// ISO 8601 payment date.
    #[serde(default)]
    pub payment_date: Option<String>,
    /// ISO 8601 redeem date.
    #[serde(default)]
    pub redeem_date: Option<String>,
    /// Additional metadata.
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
    /// Payout details.
    #[serde(default)]
    pub payout: Option<serde_json::Value>,
    /// ISO 8601 last updated timestamp.
    #[serde(default)]
    pub updated_date: Option<String>,
    /// Transaction status.
    #[serde(default)]
    pub status: Option<String>,
    /// Delivery status.
    #[serde(default)]
    pub delivery_status: Option<String>,
}

/// Public profile verification info.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileVerification {
    /// Verification status.
    #[serde(default)]
    pub status: Option<String>,
}

/// Public profile payment data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfilePaymentData {
    /// Interledger wallet address.
    #[serde(default)]
    pub interledger_wallet_address: Option<String>,
}

/// Public profile of a Chimoney user.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicProfile {
    /// Unique user identifier.
    #[serde(default)]
    pub id: Option<String>,
    /// Secondary currencies enabled on the account.
    #[serde(default)]
    pub account_second_currencies: Option<Vec<String>>,
    /// Whether preferred exchange rate is enabled.
    #[serde(default)]
    pub preferred_exchange_rate: Option<bool>,
    /// ISO 8601 account creation date.
    #[serde(default)]
    pub created_date: Option<String>,
    /// ISO 8601 join date.
    #[serde(default)]
    pub join_date: Option<String>,
    /// User email address.
    #[serde(default)]
    pub email: Option<String>,
    /// Verification information.
    #[serde(default)]
    pub verification: Option<ProfileVerification>,
    /// User's first name.
    #[serde(default)]
    pub first_name: Option<String>,
    /// User's last name.
    #[serde(default)]
    pub last_name: Option<String>,
    /// User's career or occupation.
    #[serde(default)]
    pub career: Option<String>,
    /// Reason for using Chimoney.
    #[serde(default)]
    pub purpose_of_chimoney: Option<String>,
    /// Social media profiles.
    #[serde(default)]
    pub socials: Option<serde_json::Value>,
    /// ISO 8601 last updated timestamp.
    #[serde(default)]
    pub updated_date: Option<String>,
    /// Profile ID.
    #[serde(default)]
    pub p_id: Option<i64>,
    /// Accumulated points.
    #[serde(default)]
    pub points: Option<i64>,
    /// Earned badges.
    #[serde(default)]
    pub badges: Option<Vec<String>>,
    /// Payment data.
    #[serde(default)]
    pub payment_data: Option<ProfilePaymentData>,
}
