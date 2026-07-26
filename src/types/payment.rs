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
    /// Unique identifier for the payment.
    pub id: String,
    /// Current status of the payment.
    pub status: String,
    /// URL to redirect the user to for checkout.
    #[serde(default)]
    pub checkout_url: Option<String>,
    /// Optional message providing additional details.
    #[serde(default)]
    pub message: Option<String>,
}

/// Payment verification result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentVerification {
    /// Unique identifier for the payment.
    pub id: String,
    /// Current status of the payment.
    pub status: String,
    /// Payment amount, if available.
    #[serde(default)]
    pub amount: Option<f64>,
    /// Currency code for the payment amount.
    #[serde(default)]
    pub currency: Option<String>,
    /// Optional message providing additional details.
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
    /// URL to the generated payment link.
    #[serde(default)]
    pub payment_link: Option<String>,
    /// Error message, if the simulation failed.
    #[serde(default)]
    pub error: Option<String>,
    /// List of payment entries returned by the simulation.
    #[serde(default)]
    pub data: Option<Vec<serde_json::Value>>,
    /// Any additional fields not covered by the struct.
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

/// Data returned from the simulate funding endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimulateFundingData {
    /// Whether the funding was successfully simulated.
    #[serde(default)]
    pub simulated: Option<bool>,
    /// Amount that was simulated, if applicable.
    #[serde(default)]
    pub simulated_amount: Option<f64>,
    /// Funding rail used in the simulation.
    #[serde(default)]
    pub rail: Option<String>,
}
