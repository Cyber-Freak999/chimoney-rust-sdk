use serde::{Deserialize, Serialize};

/// Passport action to perform.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PassportAction {
    /// Check the status of an existing passport.
    Check,
    /// Create a new passport.
    Create,
    /// Resend a passport claim email.
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
    /// No passport exists for this account.
    Missing,
    /// Passport exists but has not been claimed.
    Unclaimed,
    /// Passport has been claimed by the user.
    Claimed,
}

/// Passport data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PassportData {
    /// Current status of the passport.
    #[serde(default)]
    pub passport_status: Option<PassportStatus>,
    /// Unique identifier for the APort passport.
    #[serde(default)]
    pub aport_passport_id: Option<String>,
    /// Whether the passport has been claimed.
    #[serde(default)]
    pub claimed: Option<bool>,
    /// Raw passport payload from the API.
    #[serde(default)]
    pub passport: Option<serde_json::Value>,
    /// Whether a claim email was resent successfully.
    #[serde(default)]
    pub claim_email_resent: Option<bool>,
}

/// Response from passport operations.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PassportResponse {
    /// Response status string (e.g. `"success"`).
    pub status: String,
    /// Passport data, present when the request succeeds.
    #[serde(default)]
    pub data: Option<PassportData>,
}
