use serde::{Deserialize, Serialize};

/// Request to initiate a payment.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentRequest {
    /// Payer's email address.
    pub email: String,
    /// Amount in USD.
    pub amount: f64,
    /// Redirect URL after payment.
    pub redirect_url: String,
    /// Optional sub-account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
}

/// Response from initiating a payment.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentResponse {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub checkout_url: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

/// Payment verification result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentVerification {
    pub id: String,
    pub status: String,
    #[serde(default)]
    pub amount: Option<f64>,
    #[serde(default)]
    pub currency: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
}

/// Request to simulate funding via a specified rail (staging only).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateFundingRequest {
    /// The funding rail (e.g., "interac", "spei").
    pub rail: String,
    /// Reference (issueID, t_id, or p_id).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    /// Amount (used when no reference is provided).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// CLABE number (SPEI only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clabe: Option<String>,
}

/// Data returned from the simulate payment endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulatePaymentData {
    #[serde(default)]
    pub payment_link: Option<String>,
    #[serde(default)]
    pub error: Option<String>,
    #[serde(default)]
    pub data: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Data returned from the simulate funding endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateFundingData {
    #[serde(default)]
    pub simulated: Option<bool>,
    #[serde(default)]
    pub simulated_amount: Option<f64>,
    #[serde(default)]
    pub rail: Option<String>,
}
