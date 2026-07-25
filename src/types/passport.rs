use serde::{Deserialize, Serialize};

/// Passport action to perform.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PassportAction {
    Check,
    Create,
    Resend,
}

impl std::fmt::Display for PassportAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Check => write!(f, "check"),
            Self::Create => write!(f, "create"),
            Self::Resend => write!(f, "resend"),
        }
    }
}

/// Request to manage APort passport.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PassportRequest {
    /// Action to perform.
    pub action: PassportAction,
    /// Account ID (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

/// Passport status.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PassportStatus {
    Missing,
    Unclaimed,
    Claimed,
}

/// Passport data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PassportData {
    #[serde(default)]
    pub passport_status: Option<PassportStatus>,
    #[serde(default)]
    pub aport_passport_id: Option<String>,
    #[serde(default)]
    pub claimed: Option<bool>,
    #[serde(default)]
    pub passport: Option<serde_json::Value>,
    #[serde(default)]
    pub claim_email_resent: Option<bool>,
}

/// Response from passport operations.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PassportResponse {
    pub status: String,
    #[serde(default)]
    pub data: Option<PassportData>,
}
