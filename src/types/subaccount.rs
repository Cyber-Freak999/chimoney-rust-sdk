use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Sub-account details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccount {
    /// Unique identifier for the sub-account.
    pub id: String,
    /// Display name of the sub-account.
    pub name: String,
    /// Email address associated with the sub-account.
    #[serde(default)]
    pub email: Option<String>,
    /// First name of the sub-account holder.
    #[serde(default)]
    pub first_name: Option<String>,
    /// Last name of the sub-account holder.
    #[serde(default)]
    pub last_name: Option<String>,
    /// Phone number associated with the sub-account.
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
    /// Response status indicating success or failure.
    pub status: String,
    /// Human-readable message describing the result.
    #[serde(default)]
    pub message: Option<String>,
    /// Response payload.
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

/// List of sub-accounts.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountList {
    /// Response status indicating success or failure.
    pub status: String,
    /// List of sub-account details.
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
    /// Unique identifier of the member.
    pub uid: Option<String>,
    /// Full name of the member.
    pub name: Option<String>,
    /// First name of the member.
    pub first_name: Option<String>,
    /// Last name of the member.
    pub last_name: Option<String>,
    /// Email address of the member.
    pub email: Option<String>,
    /// Phone number of the member.
    pub phone_number: Option<String>,
    /// Whether the member's identity is verified.
    pub verified: Option<bool>,
    /// Whether the member is a sub-account.
    pub sub_account: Option<bool>,
    /// Parent account ID.
    pub p_id: Option<i64>,
    /// Number of points earned by the member.
    pub points: Option<i64>,
    /// Badges earned by the member.
    pub badges: Option<Vec<String>>,
    /// Communities the member belongs to.
    pub communities: Option<Vec<CommunityMembership>>,
    /// Date the member account was created.
    pub created_date: Option<String>,
    /// Date the member joined the community.
    pub join_date: Option<String>,
}

/// Community membership info within a member.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityMembership {
    /// Community type ("free" or "paid").
    pub community_type: Option<String>,
    /// Name of the community.
    pub name: Option<String>,
    /// Member's display name within the community.
    pub member_name: Option<String>,
    /// Community ID.
    #[serde(rename = "communityID")]
    pub id: Option<i64>,
    /// Membership identifier.
    pub membership_id: Option<String>,
    /// Whether the member has claimed a voucher.
    pub has_claimed_voucher: Option<bool>,
    /// Voucher code associated with the membership.
    pub voucher_code: Option<String>,
}

/// Response from community operations.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityResponse {
    /// Response status indicating success or failure.
    pub status: String,
    /// Human-readable message describing the result.
    #[serde(default)]
    pub message: Option<String>,
    /// Response payload.
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

/// Response containing community members.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommunityMembersResponse {
    /// Response status indicating success or failure.
    pub status: String,
    /// Human-readable message describing the result.
    #[serde(default)]
    pub message: Option<String>,
    /// List of community member details.
    #[serde(default)]
    pub data: Option<Vec<CommunityMember>>,
}

/// A single sub-account detail item (list variant).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubAccountListItem {
    /// Unique identifier for the sub-account.
    #[serde(default)]
    pub id: Option<String>,
    /// Last name of the sub-account holder.
    #[serde(default)]
    pub last_name: Option<String>,
    /// Parent account identifier.
    #[serde(default)]
    pub parent: Option<String>,
    /// Fee percentage applied to the sub-account.
    #[serde(default)]
    pub fee_percent: Option<i64>,
    /// Supported secondary currencies for the sub-account.
    #[serde(default)]
    pub account_second_currencies: Option<Vec<String>>,
    /// Whether the sub-account is verified.
    #[serde(default)]
    pub verified: Option<bool>,
    /// Subscription details for the sub-account.
    #[serde(default)]
    pub subscription: Option<serde_json::Value>,
    /// Whether the sub-account is a scrim user.
    #[serde(default)]
    pub is_scrim_user: Option<bool>,
    /// Whether this is a sub-account (as opposed to a primary account).
    #[serde(default)]
    pub sub_account: Option<bool>,
    /// First name of the sub-account holder.
    #[serde(default)]
    pub first_name: Option<String>,
    /// Whether a preferred exchange rate is enabled.
    #[serde(default)]
    pub preferred_exchange_rate: Option<bool>,
    /// Unique user identifier.
    #[serde(default)]
    pub uid: Option<String>,
    /// Whether the sub-account has been approved.
    #[serde(default)]
    pub approved: Option<bool>,
    /// Date the sub-account was created.
    #[serde(default)]
    pub created_date: Option<String>,
    /// Date the sub-account joined.
    #[serde(default)]
    pub join_date: Option<String>,
    /// Phone number associated with the sub-account.
    #[serde(default)]
    pub phone_number: Option<String>,
    /// Arbitrary metadata attached to the sub-account.
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
    /// Display name of the sub-account.
    #[serde(default)]
    pub name: Option<String>,
    /// Whether API access is enabled for the sub-account.
    #[serde(default)]
    pub api_use_enabled: Option<bool>,
    /// Email address associated with the sub-account.
    #[serde(default)]
    pub email: Option<String>,
    /// Parent account ID.
    #[serde(default)]
    pub p_id: Option<i64>,
    /// KYC verification status details.
    #[serde(default)]
    pub verification: Option<serde_json::Value>,
    /// Wallets associated with the sub-account.
    #[serde(default)]
    pub wallets: Option<Vec<serde_json::Value>>,
}

/// KYC link response data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KycLinkData {
    /// URL to the KYC verification page.
    pub kyc_url: String,
    /// Sub-account ID for which the KYC link was generated.
    #[serde(rename = "subAccountID")]
    pub sub_account_id: String,
    /// URL to redirect to after KYC completion.
    #[serde(default)]
    pub redirect_url: Option<String>,
}

/// Response from KYC link operation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KycLinkResponse {
    /// Response status indicating success or failure.
    pub status: String,
    /// Human-readable message describing the result.
    #[serde(default)]
    pub message: Option<String>,
    /// KYC link details.
    #[serde(default)]
    pub data: Option<KycLinkData>,
}
