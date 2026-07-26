use serde::{Deserialize, Serialize};

/// Wallet details.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Wallet {
    /// Unique identifier of the wallet.
    pub id: String,
    /// Display name of the wallet.
    #[serde(default)]
    pub name: Option<String>,
    /// Currency denomination of the wallet (e.g., "USD").
    #[serde(default)]
    pub currency: Option<String>,
    /// Current balance held in the wallet.
    #[serde(default)]
    pub balance: Option<f64>,
}

/// List of wallets response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletList {
    /// Response status indicating success or failure.
    pub status: String,
    /// List of wallets returned by the API, or `None` on failure.
    #[serde(default)]
    pub data: Option<Vec<Wallet>>,
}

/// Request to lookup a wallet.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletLookupRequest {
    /// Wallet ID.
    pub wallet_id: String,
    /// Sub-account ID.
    pub sub_account: String,
}

/// Request to transfer between wallets.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletTransferRequest {
    /// Source wallet ID.
    pub wallet: String,
    /// Amount in USD.
    pub value_in_usd: f64,
    /// Sub-account ID.
    pub sub_account: String,
    /// Receiver email or ID.
    pub receiver: String,
}

/// Response from wallet operations.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletResponse {
    /// Response status indicating success or failure.
    pub status: String,
    /// Descriptive message providing additional context about the result.
    #[serde(default)]
    pub message: Option<String>,
    /// Arbitrary response payload from the API.
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}
