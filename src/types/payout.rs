use serde::{Deserialize, Serialize};

/// Base payout request with common fields.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PayoutRequest {
    /// Sub-account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
    /// Turn off notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub turn_off_notification: Option<bool>,
}

/// Bank payout request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BankPayoutRequest {
    /// Base payout fields.
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of bank transfers.
    pub transfers: Vec<BankTransfer>,
}

/// A single bank transfer.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BankTransfer {
    /// Bank code.
    pub bank_code: String,
    /// Account number.
    pub account_number: String,
    /// Amount in USD.
    pub amount: f64,
    /// Currency.
    pub currency: String,
    /// Country code.
    pub country_code: String,
    /// Beneficiary name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beneficiary_name: Option<String>,
}

/// Airtime payout request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AirtimePayoutRequest {
    /// Base payout fields.
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of airtime transfers.
    pub transfers: Vec<AirtimeTransfer>,
}

/// A single airtime transfer.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AirtimeTransfer {
    /// Phone number.
    pub phone_number: String,
    /// Amount in USD.
    pub amount: f64,
    /// Country code.
    pub country_code: String,
}

/// Chimoney payout request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChimoneyPayoutRequest {
    /// Base payout fields.
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of Chimoney transfers.
    pub transfers: Vec<ChimoneyTransfer>,
}

/// A single Chimoney transfer.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChimoneyTransfer {
    /// Receiver email or ID.
    pub receiver: String,
    /// Amount in USD.
    pub value_in_usd: f64,
}

/// Mobile money payout request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileMoneyPayoutRequest {
    /// Base payout fields.
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of mobile money transfers.
    pub transfers: Vec<MobileMoneyTransfer>,
}

/// A single mobile money transfer.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MobileMoneyTransfer {
    /// Phone number.
    pub phone_number: String,
    /// Amount in USD.
    pub amount: f64,
    /// Country code.
    pub country_code: String,
    /// Mobile money provider code.
    pub provider_code: String,
}

/// Gift card payout request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GiftCardPayoutRequest {
    /// Base payout fields.
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of gift card transfers.
    pub transfers: Vec<GiftCardTransfer>,
}

/// A single gift card transfer.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GiftCardTransfer {
    /// Receiver email or ID.
    pub receiver: String,
    /// Amount in USD.
    pub value_in_usd: f64,
    /// Gift card provider.
    pub provider: String,
}

/// Interledger wallet payout request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InterledgerPayoutRequest {
    /// Base payout fields.
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of Interledger transfers.
    pub transfers: Vec<InterledgerTransfer>,
}

/// A single Interledger transfer.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InterledgerTransfer {
    /// Receiver address.
    pub receiver_address: String,
    /// Amount in USD.
    pub value_in_usd: f64,
}

/// Wallet payout request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletPayoutRequest {
    /// Base payout fields.
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// List of wallet transfers.
    pub transfers: Vec<WalletTransfer>,
}

/// A single wallet transfer.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletTransfer {
    /// Receiver email or ID.
    pub receiver: String,
    /// Amount in USD.
    pub value_in_usd: f64,
    /// Wallet ID.
    pub wallet_id: String,
}

/// Generic payout response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayoutResponse {
    /// Response status.
    pub status: String,
    /// Response message.
    #[serde(default)]
    pub message: Option<String>,
    /// Payout transaction ID.
    #[serde(default)]
    pub id: Option<String>,
}

/// Payout status response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PayoutStatusResponse {
    /// Response status.
    pub status: String,
    /// Response message.
    #[serde(default)]
    pub message: Option<String>,
    /// Payout status data.
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

/// Interac payout request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InteracPayoutRequest {
    /// Base payout fields.
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// Wallet currency to debit from.
    pub debit_currency: String,
    /// Interac transactions.
    pub interacs: Vec<InteracTransfer>,
}

/// A single Interac transfer.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InteracTransfer {
    /// Email to send to.
    pub email: String,
    /// Full name of the receiver.
    pub name: String,
    /// Amount in specified currency.
    pub amount: f64,
    /// Narration/description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub narration: Option<String>,
    /// Collection payment issue ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_payment_issue_id: Option<String>,
}

/// SPEI payout request (Mexican bank transfer).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeiPayoutRequest {
    /// Base payout fields.
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// Wallet currency to debit from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_currency: Option<String>,
    /// SPEI transactions.
    pub speis: Vec<SpeiTransfer>,
}

/// A single SPEI transfer.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeiTransfer {
    /// CLABE number (18 digits).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clabe: Option<String>,
    /// Phone number (10 digits).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_num: Option<String>,
    /// Debit card number (16 digits).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_card: Option<String>,
    /// Full name of the beneficiary.
    pub beneficiary: String,
    /// Amount in specified currency.
    pub amount: f64,
    /// Institution code (required when using phoneNum or debitCard).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub institution_code: Option<String>,
    /// Narration/description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub narration: Option<String>,
    /// Collection payment issue ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_payment_issue_id: Option<String>,
}

/// Request to process an unpaid transaction.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessUnpaidRequest {
    /// Sub-account ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
    /// Issue ID of the transaction to process.
    pub issue_id: String,
    /// Collection payment issue ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_payment_issue_id: Option<String>,
}

/// Canadian bill payment request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BillsCaPayoutRequest {
    /// Base payout fields.
    #[serde(flatten)]
    pub base: PayoutRequest,
    /// Wallet currency to debit from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_currency: Option<String>,
    /// Billing data.
    pub billing_data: Vec<BillingData>,
}

/// A single billing entry for Canadian bill payment.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BillingData {
    /// Email of payer.
    pub email: String,
    /// First name of payer.
    pub firstname: String,
    /// Last name of payer.
    pub lastname: String,
    /// Merchant payee code.
    pub payee_code: String,
    /// Merchant payee name.
    pub payee_name: String,
    /// Amount to pay.
    pub amount: f64,
    /// Customer account number for the payee.
    pub account_id: String,
    /// Narration/description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub narration: Option<String>,
    /// Collection payment issue ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_payment_issue_id: Option<String>,
}
