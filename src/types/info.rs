use serde::{Deserialize, Serialize};

/// Pagination metadata for list responses.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    /// Total number of records.
    pub total: i64,
    /// Number of records skipped.
    pub skip: i64,
    /// Number of records to take.
    pub take: i64,
}

/// Result of a bank search query.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankSearchResult {
    /// Unique primary key for the bank.
    pub primary_key: String,
    /// Institution name.
    pub institution_name: String,
    /// Address line 1.
    pub address1: String,
    /// Address line 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    /// City.
    pub city: String,
    /// Region/state.
    pub region: String,
    /// Country name.
    pub country: String,
    /// ISO country code.
    #[serde(rename = "countryISO")]
    pub country_iso: String,
    /// Postal code.
    pub postal_code: String,
    /// SWIFT/BIC code.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "swiftBIC")]
    pub swift_bic: Option<String>,
    /// National bank code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_bank_code: Option<String>,
    /// National bank code type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_bank_code_type: Option<String>,
    /// Office type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub office_type: Option<String>,
    /// Branch name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    /// Phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Fax number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax: Option<String>,
}

/// Response from the bank search endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BankSearchResponse {
    /// Status of the response.
    pub status: String,
    /// Response message.
    pub message: String,
    /// List of bank search results.
    pub data: Vec<BankSearchResult>,
    /// Pagination info.
    #[serde(default)]
    pub pagination: Option<Pagination>,
}

/// A single beneficiary validation rule.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeneficiaryRule {
    /// Rule name.
    pub name: String,
    /// Human-readable label.
    pub label: String,
    /// Regex pattern for validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regex_pattern: Option<String>,
    /// Allowed values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_values: Option<Vec<String>>,
    /// Default value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// Input type hint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    /// Whether the field is required.
    pub required: bool,
}

/// Response from the beneficiary rules endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BeneficiaryRulesResponse {
    /// Status of the response.
    pub status: String,
    /// Response message.
    pub message: String,
    /// List of beneficiary rules.
    pub data: Vec<BeneficiaryRule>,
}

/// A supported identification type.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentificationType {
    /// Identification type name.
    #[serde(rename = "type")]
    pub r#type: String,
    /// Type code.
    pub code: String,
}

/// Response from the identification types endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IdentificationTypesResponse {
    /// Status of the response.
    pub status: String,
    /// Response message.
    pub message: String,
    /// List of identification types.
    pub data: Vec<IdentificationType>,
}

/// Request body for fee estimation.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeEstimateRequest {
    /// Transaction amount.
    pub amount: f64,
    /// Currency code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Transfer rail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rail: Option<String>,
    /// Transfer direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<String>,
}

/// Detailed fee breakdown.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeEstimateData {
    /// Transaction amount.
    pub amount: f64,
    /// Currency code.
    pub currency: String,
    /// Transfer rail.
    #[serde(default)]
    pub rail: Option<String>,
    /// Transfer direction.
    pub direction: String,
    /// Platform fee.
    pub platform_fee: f64,
    /// Rail fee.
    pub rail_fee: f64,
    /// Total fee.
    pub total_fee: f64,
    /// Net amount after fees.
    pub net_amount: f64,
    /// Additional notes.
    #[serde(default)]
    pub note: Option<String>,
}

/// Response from the fee estimate endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeEstimateResponse {
    /// Status of the response.
    pub status: String,
    /// Response message.
    pub message: String,
    /// Fee estimate details.
    pub data: FeeEstimateData,
}

/// Request body for voucher validation.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateVoucherRequest {
    /// Voucher code to validate.
    pub code: String,
}

/// Voucher data returned on validation.
#[derive(Debug, Clone, Deserialize)]
pub struct VoucherData {
    /// Voucher code.
    #[serde(rename = "Code")]
    pub code: String,
    /// Creation date.
    #[serde(default, rename = "Created Date")]
    pub created_date: Option<String>,
    /// Delivery date.
    #[serde(default, rename = "Delivery Date")]
    pub delivery_date: Option<String>,
    /// Expiry date.
    #[serde(default, rename = "Expiry Date")]
    pub expiry_date: Option<String>,
}

/// Response from the voucher validation endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidateVoucherResponse {
    /// Status of the response.
    pub status: String,
    /// Response message.
    pub message: String,
    /// Voucher details.
    pub data: VoucherData,
}

/// Request body for searching merchants.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchMerchantsRequest {
    /// Search query.
    pub search: String,
}

/// A country state or region.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountryState {
    /// State/region code.
    pub code: String,
    /// State/region name.
    pub name: String,
}

/// Response from the country states endpoint.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountryStatesResponse {
    /// Status of the response.
    pub status: String,
    /// Response message.
    pub message: String,
    /// List of states/regions.
    pub data: Vec<CountryState>,
}
