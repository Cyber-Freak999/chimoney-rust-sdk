use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Sub-account details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccount {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub phone_number: Option<String>,
}

/// Request to create a sub-account.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubAccountRequest {
    /// Sub-account name.
    pub name: String,
    /// First name.
    pub first_name: String,
    /// Last name.
    pub last_name: String,
    /// Email address.
    pub email: String,
    /// Phone number.
    pub phone_number: String,
}

/// Request to update a sub-account.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateSubAccountRequest {
    /// Sub-account ID.
    pub id: String,
    /// First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// Metadata key-value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, String>>,
}

/// Response from sub-account operations.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

/// List of sub-accounts.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountList {
    pub status: String,
    #[serde(default)]
    pub data: Option<Vec<SubAccount>>,
}

/// Community details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Community {
    /// Community ID (integer).
    #[serde(rename = "communityID")]
    pub community_id: i64,
    /// Community name.
    pub name: String,
    /// Membership ID.
    pub membership_id: String,
    /// Member name.
    pub member_name: String,
    /// Community type ("free" or "paid").
    pub community_type: String,
    /// Voucher code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voucher_code: Option<String>,
}

/// Request to create a community under a sub-account.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCommunityRequest {
    /// Sub-account ID.
    pub id: String,
    /// Community details.
    pub community: Community,
}

/// Request to update a community under a sub-account.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCommunityRequest {
    /// Sub-account ID.
    pub id: String,
    /// Community ID.
    #[serde(rename = "communityID")]
    pub community_id: i64,
    /// Updated membership ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub membership_id: Option<String>,
    /// Updated member name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_name: Option<String>,
}

/// Community member details.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityMember {
    pub uid: Option<String>,
    pub name: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub verified: Option<bool>,
    pub sub_account: Option<bool>,
    pub p_id: Option<i64>,
    pub points: Option<i64>,
    pub badges: Option<Vec<String>>,
    pub communities: Option<Vec<CommunityMembership>>,
    pub created_date: Option<String>,
    pub join_date: Option<String>,
}

/// Community membership info within a member.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityMembership {
    pub community_type: Option<String>,
    pub name: Option<String>,
    pub member_name: Option<String>,
    #[serde(rename = "communityID")]
    pub id: Option<i64>,
    pub membership_id: Option<String>,
    pub has_claimed_voucher: Option<bool>,
    pub voucher_code: Option<String>,
}

/// Response from community operations.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

/// Response containing community members.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityMembersResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<Vec<CommunityMember>>,
}

/// A single sub-account detail item (list variant).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountListItem {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub last_name: Option<String>,
    #[serde(default)]
    pub parent: Option<String>,
    #[serde(default)]
    pub fee_percent: Option<i64>,
    #[serde(default)]
    pub account_second_currencies: Option<Vec<String>>,
    #[serde(default)]
    pub verified: Option<bool>,
    #[serde(default)]
    pub subscription: Option<serde_json::Value>,
    #[serde(default)]
    pub is_scrim_user: Option<bool>,
    #[serde(default)]
    pub sub_account: Option<bool>,
    #[serde(default)]
    pub first_name: Option<String>,
    #[serde(default)]
    pub preferred_exchange_rate: Option<bool>,
    #[serde(default)]
    pub uid: Option<String>,
    #[serde(default)]
    pub approved: Option<bool>,
    #[serde(default)]
    pub created_date: Option<String>,
    #[serde(default)]
    pub join_date: Option<String>,
    #[serde(default)]
    pub phone_number: Option<String>,
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub api_use_enabled: Option<bool>,
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub p_id: Option<i64>,
    #[serde(default)]
    pub verification: Option<serde_json::Value>,
    #[serde(default)]
    pub wallets: Option<Vec<serde_json::Value>>,
}

/// KYC link response data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KycLinkData {
    pub kyc_url: String,
    #[serde(rename = "subAccountID")]
    pub sub_account_id: String,
    #[serde(default)]
    pub redirect_url: Option<String>,
}

/// Response from KYC link operation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KycLinkResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<KycLinkData>,
}
