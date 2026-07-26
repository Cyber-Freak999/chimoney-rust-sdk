use serde::{Deserialize, Serialize};

/// A multicurrency wallet.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MulticurrencyWallet {
    /// Unique identifier of the wallet.
    pub id: String,
    /// Currency code (e.g. "USD", "NGN").
    pub currency: String,
    /// Current balance in the wallet's currency.
    #[serde(default)]
    pub balance: Option<f64>,
    /// Sub-account that owns this wallet.
    #[serde(default)]
    pub sub_account: Option<String>,
    /// Arbitrary metadata associated with the wallet.
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
    /// ISO 8601 timestamp when the wallet was created.
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Request to create a multicurrency wallet.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMulticurrencyWalletRequest {
    /// Sub-account that will own the new wallet.
    pub sub_account: String,
    /// Currency code for the new wallet.
    pub currency: String,
    /// Optional metadata to attach to the wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

/// Request to update a multicurrency wallet.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMulticurrencyWalletRequest {
    /// ID of the wallet to update.
    pub wallet_id: String,
    /// New metadata to set on the wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

/// Response from multicurrency wallet operations.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MulticurrencyWalletResponse {
    /// Response status (e.g. "success", "error").
    pub status: String,
    /// Optional human-readable message describing the result.
    #[serde(default)]
    pub message: Option<String>,
    /// The wallet returned by the operation, if any.
    #[serde(default)]
    pub data: Option<MulticurrencyWallet>,
}

/// Response from listing multicurrency wallets.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MulticurrencyWalletListResponse {
    /// Response status (e.g. "success", "error").
    pub status: String,
    /// List of wallets returned by the operation.
    #[serde(default)]
    pub data: Option<Vec<MulticurrencyWallet>>,
}

/// Request to get a transfer quote.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferQuoteRequest {
    /// ID of the source wallet.
    pub from_wallet: String,
    /// ID of the destination wallet.
    pub to_wallet: String,
    /// Amount to transfer in the source currency.
    pub amount: f64,
    /// Currency code of the source wallet.
    pub from_currency: String,
    /// Currency code of the destination wallet.
    pub to_currency: String,
}

/// Response from a transfer quote request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferQuoteResponse {
    /// Response status (e.g. "success", "error").
    pub status: String,
    /// Exchange rate applied to the transfer.
    #[serde(default)]
    pub exchange_rate: Option<f64>,
    /// Fee charged for the transfer.
    #[serde(default)]
    pub fee: Option<f64>,
    /// Amount debited from the source wallet.
    #[serde(default)]
    pub source_amount: Option<f64>,
    /// Amount credited to the destination wallet.
    #[serde(default)]
    pub destination_amount: Option<f64>,
}

/// Request to transfer between multicurrency wallets.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MulticurrencyTransferRequest {
    /// ID of the source wallet.
    pub from_wallet: String,
    /// Recipient identifier (e.g. email or phone).
    pub recipient: String,
    /// Amount to transfer in the source currency.
    pub amount: f64,
    /// Currency code of the source wallet.
    pub from_currency: String,
    /// Currency code of the destination wallet.
    pub to_currency: String,
    /// Optional note attached to the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

/// Response from a multicurrency transfer.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MulticurrencyTransferResponse {
    /// Response status (e.g. "success", "error").
    pub status: String,
    /// Unique identifier of the completed transaction.
    #[serde(default)]
    pub transaction_id: Option<String>,
    /// Optional human-readable message describing the result.
    #[serde(default)]
    pub message: Option<String>,
}

/// Request to issue a multicurrency wallet.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueWalletRequest {
    /// Sub-account that will own the issued wallet.
    pub sub_account: String,
    /// Currency code for the wallet to issue.
    pub currency: String,
}

/// Request to issue a bank account.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueBankAccountRequest {
    /// Sub-account that will own the issued bank account.
    pub sub_account: String,
    /// ISO 3166-1 alpha-2 country code of the bank.
    pub country_code: String,
    /// Bank routing or sort code.
    pub bank_code: String,
    /// Bank account number.
    pub account_number: String,
}
