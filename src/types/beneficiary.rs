use serde::{Deserialize, Serialize};

/// A beneficiary for payouts.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Beneficiary {
    /// Unique identifier for the beneficiary.
    pub id: String,
    /// Type of beneficiary (e.g., bank, mobile money).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub beneficiary_type: Option<String>,
    /// Bank account number.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub account_number: Option<String>,
    /// Bank identifier code.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bank_code: Option<String>,
    /// ISO country code.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub country_code: Option<String>,
    /// Name of the beneficiary.
    pub name: String,
    /// Timestamp when the beneficiary was created.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub created_at: Option<String>,
}

/// Request to create a bank beneficiary.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBankBeneficiaryRequest {
    /// Bank account number.
    pub account_number: String,
    /// Bank identifier code.
    pub bank_code: String,
    /// ISO country code.
    pub country_code: String,
    /// Name of the beneficiary.
    pub name: String,
    /// Currency code (e.g., USD, NGN).
    pub currency: String,
}

/// Response from creating or getting a beneficiary.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeneficiaryResponse {
    /// Response status (e.g., success, error).
    pub status: String,
    /// Optional response message.
    #[serde(default)]
    pub message: Option<String>,
    /// Optional beneficiary data.
    #[serde(default)]
    pub data: Option<Beneficiary>,
}

/// Response from listing beneficiaries.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeneficiaryListResponse {
    /// Response status (e.g., success, error).
    pub status: String,
    /// Optional list of beneficiaries.
    #[serde(default)]
    pub data: Option<Vec<Beneficiary>>,
}

/// Request to preview a transfer to a beneficiary.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewTransferRequest {
    /// Identifier of the beneficiary to transfer to.
    pub beneficiary_id: String,
    /// Amount to transfer.
    pub amount: f64,
    /// Currency code (e.g., USD, NGN).
    pub currency: String,
}

/// Response from previewing a transfer.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviewTransferResponse {
    /// Response status (e.g., success, error).
    pub status: String,
    /// Optional transfer fee.
    #[serde(default)]
    pub fee: Option<f64>,
    /// Optional exchange rate.
    #[serde(default)]
    pub exchange_rate: Option<f64>,
    /// Optional total amount including fees.
    #[serde(default)]
    pub total_amount: Option<f64>,
    /// Optional amount received at destination.
    #[serde(default)]
    pub destination_amount: Option<f64>,
}
