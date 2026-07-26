use std::fmt;

use reqwest_middleware::ClientWithMiddleware;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::{ChimoneyError, Result};
use crate::middleware::{build_client, DEFAULT_MAX_RETRIES, DEFAULT_TIMEOUT_SECS};

/// Chimoney API client.
///
/// # Example
///
/// ```rust
/// use chimoney_rust_sdk::ChimoneyClient;
///
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = ChimoneyClient::new("your_api_key")?;
/// # Ok(())
/// # }
/// ```
pub struct ChimoneyClient {
    client: ClientWithMiddleware,
    api_key: String,
    base_url: String,
}

enum Method {
    Get,
    Post,
    Delete,
    Patch,
}

impl ChimoneyClient {
    /// Create a new ChimoneyClient with default settings.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::ApiKeyEmpty`] if the API key is empty.
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::builder(api_key).build()
    }

    /// Create a new ChimoneyClient with sandbox URL.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::ApiKeyEmpty`] if the API key is empty.
    pub fn new_sandbox(api_key: impl Into<String>) -> Result<Self> {
        Self::builder(api_key)
            .base_url("https://api-v2-sandbox.chimoney.io")
            .build()
    }

    /// Get a builder for configuring the client.
    pub fn builder(api_key: impl Into<String>) -> ChimoneyClientBuilder {
        ChimoneyClientBuilder {
            api_key: api_key.into(),
            base_url: "https://api.chimoney.io".to_string(),
            max_retries: DEFAULT_MAX_RETRIES,
            timeout_secs: DEFAULT_TIMEOUT_SECS,
        }
    }

    /// Get a reference to the HTTP client.
    pub fn http_client(&self) -> &ClientWithMiddleware {
        &self.client
    }

    /// Get the base URL.
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    async fn request(
        &self,
        method: Method,
        path: &str,
        body: Option<String>,
        query: Option<&str>,
    ) -> Result<String> {
        let mut url = format!("{}{}", self.base_url, path);
        if let Some(params) = query {
            url.push('?');
            url.push_str(params);
        }

        log::debug!("{} {}", match method {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Delete => "DELETE",
            Method::Patch => "PATCH",
        }, url);

        let mut req = match method {
            Method::Get => self.client.get(&url),
            Method::Post => self.client.post(&url),
            Method::Delete => self.client.delete(&url),
            Method::Patch => self.client.patch(&url),
        };

        req = req
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("X-API-KEY", &self.api_key);

        if let Some(body) = body {
            req = req.body(body);
        }

        let response = req.send().await.map_err(ChimoneyError::MiddlewareError)?;
        log::debug!("Response {} {}", response.status(), url);
        self.handle_response(response).await
    }

    async fn post_json<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        request: &T,
        query: Option<&str>,
    ) -> Result<R> {
        let body =
            serde_json::to_string(request).map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.request(Method::Post, path, Some(body), query).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        serde_json::from_value(json).map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    async fn post_json_data<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        request: &T,
        query: Option<&str>,
    ) -> Result<R> {
        let body =
            serde_json::to_string(request).map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.request(Method::Post, path, Some(body), query).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        serde_json::from_value(json["data"].clone())
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    async fn get_json<R: DeserializeOwned>(&self, path: &str, query: Option<&str>) -> Result<R> {
        let response = self.request(Method::Get, path, None, query).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        serde_json::from_value(json).map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    async fn get_json_data<R: DeserializeOwned>(
        &self,
        path: &str,
        query: Option<&str>,
    ) -> Result<R> {
        let response = self.request(Method::Get, path, None, query).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        serde_json::from_value(json["data"].clone())
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    async fn delete_json<R: DeserializeOwned>(&self, path: &str, query: Option<&str>) -> Result<R> {
        let response = self.request(Method::Delete, path, None, query).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        serde_json::from_value(json).map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    async fn patch_json<T: Serialize, R: DeserializeOwned>(
        &self,
        path: &str,
        request: &T,
        query: Option<&str>,
    ) -> Result<R> {
        let body =
            serde_json::to_string(request).map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        let response = self.request(Method::Patch, path, Some(body), query).await?;
        let json: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| ChimoneyError::ParseError(e.to_string()))?;
        serde_json::from_value(json).map_err(|e| ChimoneyError::ParseError(e.to_string()))
    }

    // ── Account Methods ──────────────────────────────────────────────

    /// Get transactions by account ID.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_transactions(
        &self,
        account_id: &str,
    ) -> Result<Vec<crate::types::Transaction>> {
        let path = "/v0.2.4/accounts/transactions";
        let body = serde_json::json!({ "subAccount": account_id });
        self.post_json_data(path, &body, None).await
    }

    /// Get single transaction details.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_transaction(&self, transaction_id: &str) -> Result<crate::types::Transaction> {
        let path = "/v0.2.4/accounts/transaction";
        let body = serde_json::json!({ "id": transaction_id });
        self.post_json_data(path, &body, None).await
    }

    /// Get transaction by issue ID.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_issue_id_transaction(
        &self,
        issue_id: &str,
    ) -> Result<Vec<crate::types::IssueIdTransactionItem>> {
        let path = "/v0.2.4/accounts/issue-id-transactions";
        let body = serde_json::json!({ "issueID": issue_id });
        self.post_json_data(path, &body, None).await
    }

    /// Get public profile.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_public_profile(&self) -> Result<crate::types::PublicProfile> {
        let path = "/v0.2.4/accounts/public-profile";
        let body = serde_json::json!({});
        self.post_json_data(path, &body, None).await
    }

    /// Transfer between accounts.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn transfer(
        &self,
        request: &crate::types::TransferRequest,
    ) -> Result<crate::types::TransferResponse> {
        self.post_json("/v0.2.4/accounts/transfer", request, None)
            .await
    }

    /// Initiate Chimoney transaction.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn initiate_chimoney(
        &self,
        request: &crate::types::InitiateChimoneyRequest,
    ) -> Result<crate::types::InitiateChimoneyResponse> {
        self.post_json("/v0.2.4/payouts/initiate-chimoney", request, None)
            .await
    }

    /// Delete unpaid transactions.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn delete_unpaid_transactions(
        &self,
        chi_ref: &str,
    ) -> Result<crate::types::DeleteUnpaidTransactionResponse> {
        let path = "/v0.2.4/accounts/delete-unpaid-transaction";
        let query = format!("chiRef={}", chi_ref);
        self.delete_json(path, Some(&query)).await
    }

    /// Issue an Interledger wallet address for a user.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn issue_wallet_address(
        &self,
        request: &crate::types::IssueWalletAddressRequest,
    ) -> Result<crate::types::AccountOperationResponse> {
        self.post_json("/v0.2.4/accounts/issue-wallet-address", request, None)
            .await
    }

    /// Claim community membership reward.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn claim_reward(
        &self,
        request: &crate::types::ClaimRewardRequest,
    ) -> Result<crate::types::AccountOperationResponse> {
        self.post_json("/v0.2.4/accounts/claim-reward", request, None)
            .await
    }

    // ── Payment Methods ─────────────────────────────────────────────

    /// Initiate a payment.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn initiate_payment(
        &self,
        request: &crate::types::PaymentRequest,
    ) -> Result<crate::types::PaymentResponse> {
        self.post_json("/v0.2.4/payment/initiate", request, None)
            .await
    }

    /// Verify a payment.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn verify_payment(
        &self,
        issue_id: &str,
    ) -> Result<crate::types::PaymentVerification> {
        let path = "/v0.2.4/payment/verify";
        let body = serde_json::json!({ "issueID": issue_id });
        self.post_json_data(path, &body, None).await
    }

    /// Simulate a payment (sandbox only).
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn simulate_payment(
        &self,
        issue_id: &str,
    ) -> Result<crate::types::SimulatePaymentData> {
        let path = "/v0.2.4/payment/simulate";
        let body = serde_json::json!({ "issueID": issue_id });
        self.post_json_data(path, &body, None).await
    }

    /// Simulate funding via a specified rail (staging only).
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn simulate_funding(
        &self,
        request: &crate::types::SimulateFundingRequest,
    ) -> Result<crate::types::SimulateFundingData> {
        let path = "/v0.2.4/payment/simulate-funding";
        self.post_json_data(path, request, None).await
    }

    // ── Payout Methods ─────────────────────────────────────────────

    /// Payout via bank transfer.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn payout_bank(
        &self,
        request: &crate::types::BankPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        self.post_json("/v0.2.4/payouts/bank", request, None).await
    }

    /// Payout via airtime.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn payout_airtime(
        &self,
        request: &crate::types::AirtimePayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        self.post_json("/v0.2.4/payouts/airtime", request, None)
            .await
    }

    /// Payout via Chimoney.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn payout_chimoney(
        &self,
        request: &crate::types::ChimoneyPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        self.post_json("/v0.2.4/payouts/chimoney", request, None)
            .await
    }

    /// Payout via mobile money.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn payout_mobile_money(
        &self,
        request: &crate::types::MobileMoneyPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        self.post_json("/v0.2.4/payouts/mobile-money", request, None)
            .await
    }

    /// Payout via gift card.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn payout_giftcard(
        &self,
        request: &crate::types::GiftCardPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        self.post_json("/v0.2.4/payouts/gift-card", request, None)
            .await
    }

    /// Payout via Interledger wallet.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn payout_interledger(
        &self,
        request: &crate::types::InterledgerPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        self.post_json("/v0.2.4/payouts/interledger-wallet", request, None)
            .await
    }

    /// Payout via wallet.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn payout_wallet(
        &self,
        request: &crate::types::WalletPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        self.post_json("/v0.2.4/payouts/wallet", request, None)
            .await
    }

    /// Check payout status.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn check_payout_status(
        &self,
        chi_ref: &str,
    ) -> Result<crate::types::PayoutStatusResponse> {
        let path = "/v0.2.4/payouts/status";
        let body = serde_json::json!({ "chiRef": chi_ref });
        self.post_json(path, &body, None).await
    }

    /// Payout via Interac e-Transfer (Canada).
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn payout_interac(
        &self,
        request: &crate::types::InteracPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        self.post_json("/v0.2.4/payouts/interac", request, None)
            .await
    }

    /// Payout via SPEI (Mexican bank transfer).
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn payout_spei(
        &self,
        request: &crate::types::SpeiPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        self.post_json("/v0.2.4/payouts/spei", request, None).await
    }

    /// Debit wallet to process an unpaid transaction.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn process_unpaid(
        &self,
        request: &crate::types::ProcessUnpaidRequest,
    ) -> Result<crate::types::PayoutResponse> {
        self.post_json("/v0.2.4/payouts/process", request, None)
            .await
    }

    /// Payout Canadian bill payment.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn payout_bills_ca(
        &self,
        request: &crate::types::BillsCaPayoutRequest,
    ) -> Result<crate::types::PayoutResponse> {
        self.post_json("/v0.2.4/payouts/bills/ca", request, None)
            .await
    }

    // ── Agent Methods ─────────────────────────────────────────────

    /// Create a new agent.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn create_agent(
        &self,
        request: &crate::types::CreateAgentRequest,
    ) -> Result<crate::types::AgentResponse> {
        self.post_json("/v0.2.4/agents/create", request, None).await
    }

    /// List all agents.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn list_agents(&self) -> Result<crate::types::AgentListResponse> {
        self.get_json("/v0.2.4/agents/list", None).await
    }

    /// Get agent details by ID.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_agent(&self, agent_id: &str) -> Result<crate::types::AgentResponse> {
        let path = "/v0.2.4/agents/get";
        let query = format!("agentId={}", agent_id);
        self.get_json(path, Some(&query)).await
    }

    /// Update an agent.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn update_agent(
        &self,
        request: &crate::types::UpdateAgentRequest,
    ) -> Result<crate::types::AgentResponse> {
        self.patch_json("/v0.2.4/agents/update", request, None)
            .await
    }

    /// Update agent policies.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn update_agent_policies(
        &self,
        request: &crate::types::UpdateAgentPoliciesRequest,
    ) -> Result<crate::types::AgentResponse> {
        self.patch_json("/v0.2.4/agents/update-policies", request, None)
            .await
    }

    /// Suspend an agent.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn suspend_agent(
        &self,
        request: &crate::types::AgentIdRequest,
    ) -> Result<crate::types::AgentResponse> {
        self.post_json("/v0.2.4/agents/suspend", request, None)
            .await
    }

    /// Reactivate a suspended agent.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn reactivate_agent(
        &self,
        request: &crate::types::AgentIdRequest,
    ) -> Result<crate::types::AgentResponse> {
        self.post_json("/v0.2.4/agents/reactivate", request, None)
            .await
    }

    /// Get API key for an agent.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_agent_api_key(
        &self,
        agent_id: &str,
    ) -> Result<crate::types::AgentApiKeyResponse> {
        let path = "/v0.2.4/agents/api-key";
        let query = format!("agentId={}", agent_id);
        self.get_json(path, Some(&query)).await
    }

    /// Manage an agent API key (create, rotate, revoke, delete).
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn manage_agent_api_key(
        &self,
        request: &crate::types::ManageAgentApiKeyRequest,
    ) -> Result<crate::types::AgentApiKeyResponse> {
        self.post_json("/v0.2.4/agents/api-key", request, None)
            .await
    }

    /// Get transactions for an agent.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_agent_transactions(
        &self,
        request: &crate::types::AgentTransactionsRequest,
    ) -> Result<crate::types::AgentTransactionsResponse> {
        self.post_json("/v0.2.4/agents/transactions", request, None)
            .await
    }

    /// Fund an agent wallet.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn fund_agent(
        &self,
        request: &crate::types::FundAgentRequest,
    ) -> Result<crate::types::AgentResponse> {
        self.post_json("/v0.2.4/agents/fund", request, None).await
    }

    /// Get agent capabilities, limits, and regions configuration.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_agent_capabilities_limits(
        &self,
    ) -> Result<crate::types::AgentCapabilitiesLimitsResponse> {
        self.get_json("/v0.2.4/agents/capabilities-limits", None)
            .await
    }

    // ── Beneficiary Methods ─────────────────────────────────────────

    /// Get all beneficiaries.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_beneficiaries(&self) -> Result<crate::types::BeneficiaryListResponse> {
        self.get_json("/v0.2.4/beneficiary", None).await
    }

    /// Create a bank beneficiary.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn create_bank_beneficiary(
        &self,
        request: &crate::types::CreateBankBeneficiaryRequest,
    ) -> Result<crate::types::BeneficiaryResponse> {
        self.post_json("/v0.2.4/beneficiary/bank", request, None)
            .await
    }

    /// Preview a transfer to a beneficiary.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn preview_transfer(
        &self,
        request: &crate::types::PreviewTransferRequest,
    ) -> Result<crate::types::PreviewTransferResponse> {
        self.post_json("/v0.2.4/beneficiary/preview", request, None)
            .await
    }

    // ── Redeem Methods ─────────────────────────────────────────────

    /// Redeem airtime.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn redeem_airtime(
        &self,
        request: &crate::types::RedeemAirtimeRequest,
    ) -> Result<crate::types::RedeemResponse> {
        self.post_json("/v0.2.4/redeem/airtime", request, None)
            .await
    }

    /// Redeem Chimoney.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn redeem_chimoney(
        &self,
        request: &crate::types::RedeemChimoneyRequest,
    ) -> Result<crate::types::RedeemResponse> {
        self.post_json("/v0.2.4/redeem/chimoney", request, None)
            .await
    }

    /// Redeem gift card.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn redeem_giftcard(
        &self,
        request: &crate::types::RedeemGiftCardRequest,
    ) -> Result<crate::types::RedeemResponse> {
        self.post_json("/v0.2.4/redeem/gift-card", request, None)
            .await
    }

    /// Redeem mobile money.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn redeem_mobile_money(
        &self,
        request: &crate::types::RedeemMobileMoneyRequest,
    ) -> Result<crate::types::RedeemResponse> {
        self.post_json("/v0.2.4/redeem/mobile-money", request, None)
            .await
    }

    /// Redeem with custom data.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn redeem_any(
        &self,
        request: &crate::types::RedeemAnyRequest,
    ) -> Result<crate::types::RedeemResponse> {
        self.post_json("/v0.2.4/redeem/any", request, None).await
    }

    // ── SubAccount Methods ─────────────────────────────────────────

    /// Create a sub-account.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn create_sub_account(
        &self,
        request: &crate::types::CreateSubAccountRequest,
    ) -> Result<crate::types::SubAccountResponse> {
        self.post_json("/v0.2.4/sub-account/create", request, None)
            .await
    }

    /// Update a sub-account.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn update_sub_account(
        &self,
        request: &crate::types::UpdateSubAccountRequest,
    ) -> Result<crate::types::SubAccountResponse> {
        self.post_json("/v0.2.4/sub-account/update", request, None)
            .await
    }

    /// Delete a sub-account.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn delete_sub_account(
        &self,
        sub_account_id: &str,
    ) -> Result<crate::types::SubAccountResponse> {
        let path = "/v0.2.4/sub-account/delete";
        let query = format!("id={}", sub_account_id);
        self.delete_json(path, Some(&query)).await
    }

    /// Get sub-account details.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_sub_account(
        &self,
        sub_account_id: &str,
    ) -> Result<crate::types::SubAccountListItem> {
        let path = "/v0.2.4/sub-account/get";
        let query = format!("id={}", sub_account_id);
        self.get_json_data(path, Some(&query)).await
    }

    /// List all sub-accounts.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn list_sub_accounts(&self) -> Result<Vec<crate::types::SubAccountListItem>> {
        let path = "/v0.2.4/sub-account/list";
        self.get_json_data(path, None).await
    }

    // ── Community Methods ────────────────────────────────────────────

    /// Create a community under a sub-account.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn create_community(
        &self,
        request: &crate::types::CreateCommunityRequest,
    ) -> Result<crate::types::CommunityResponse> {
        self.post_json("/v0.2.4/sub-account/community/create", request, None)
            .await
    }

    /// Update a community under a sub-account.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn update_community(
        &self,
        request: &crate::types::UpdateCommunityRequest,
    ) -> Result<crate::types::CommunityResponse> {
        self.post_json("/v0.2.4/sub-account/community/update", request, None)
            .await
    }

    /// Get community members.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_community_members(
        &self,
        community_id: i64,
        limit: Option<i32>,
        start_after_id: Option<&str>,
        start_before_id: Option<&str>,
    ) -> Result<crate::types::CommunityMembersResponse> {
        let path = "/v0.2.4/sub-account/community/members";
        let mut query = format!("communityID={}", community_id);
        if let Some(l) = limit {
            query.push_str(&format!("&limit={}", l));
        }
        if let Some(id) = start_after_id {
            query.push_str(&format!("&startAfterId={}", id));
        }
        if let Some(id) = start_before_id {
            query.push_str(&format!("&startBeforeId={}", id));
        }
        self.get_json(path, Some(&query)).await
    }

    /// Get KYC verification page link for a sub-account.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_kyc_link(
        &self,
        sub_account_id: &str,
        redirect_url: Option<&str>,
    ) -> Result<crate::types::KycLinkResponse> {
        let path = "/v0.2.4/sub-account/kyc/link";
        let mut query = format!("subAccountID={}", sub_account_id);
        if let Some(url) = redirect_url {
            query.push_str(&format!("&redirectUrl={}", url));
        }
        self.get_json(path, Some(&query)).await
    }

    // ── Wallet Methods ─────────────────────────────────────────────

    /// List wallets.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn list_wallets(&self, sub_account: &str) -> Result<crate::types::WalletList> {
        let path = "/v0.2.4/wallets/list";
        let body = serde_json::json!({ "subAccount": sub_account });
        self.post_json(path, &body, None).await
    }

    /// Lookup a wallet.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn lookup_wallet(
        &self,
        request: &crate::types::WalletLookupRequest,
    ) -> Result<crate::types::WalletResponse> {
        self.post_json("/v0.2.4/wallets/lookup", request, None)
            .await
    }

    /// Transfer between wallets.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn transfer_between_wallets(
        &self,
        request: &crate::types::WalletTransferRequest,
    ) -> Result<crate::types::WalletResponse> {
        self.post_json("/v0.2.4/wallets/transfer", request, None)
            .await
    }

    // ── Multicurrency Wallet Methods ────────────────────────────────

    /// Create a multicurrency wallet.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn create_multicurrency_wallet(
        &self,
        request: &crate::types::CreateMulticurrencyWalletRequest,
    ) -> Result<crate::types::MulticurrencyWalletResponse> {
        self.post_json("/v0.2.4/multicurrency-wallets/create", request, None)
            .await
    }

    /// Update a multicurrency wallet.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn update_multicurrency_wallet(
        &self,
        request: &crate::types::UpdateMulticurrencyWalletRequest,
    ) -> Result<crate::types::MulticurrencyWalletResponse> {
        self.patch_json("/v0.2.4/multicurrency-wallets/update", request, None)
            .await
    }

    /// Get a multicurrency wallet by ID.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_multicurrency_wallet(
        &self,
        wallet_id: &str,
    ) -> Result<crate::types::MulticurrencyWalletResponse> {
        let path = "/v0.2.4/multicurrency-wallets/get";
        let query = format!("walletId={}", wallet_id);
        self.get_json(path, Some(&query)).await
    }

    /// List all multicurrency wallets.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn list_multicurrency_wallets(
        &self,
    ) -> Result<crate::types::MulticurrencyWalletListResponse> {
        self.get_json("/v0.2.4/multicurrency-wallets/list", None)
            .await
    }

    /// Get a transfer quote.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_transfer_quote(
        &self,
        request: &crate::types::TransferQuoteRequest,
    ) -> Result<crate::types::TransferQuoteResponse> {
        self.post_json(
            "/v0.2.4/multicurrency-wallets/transfer-quote",
            request,
            None,
        )
        .await
    }

    /// Transfer between multicurrency wallets.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn transfer_multicurrency(
        &self,
        request: &crate::types::MulticurrencyTransferRequest,
    ) -> Result<crate::types::MulticurrencyTransferResponse> {
        self.post_json("/v0.2.4/multicurrency-wallets/transfer", request, None)
            .await
    }

    /// Issue a multicurrency wallet.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn issue_multicurrency_wallet(
        &self,
        request: &crate::types::IssueWalletRequest,
    ) -> Result<crate::types::MulticurrencyWalletResponse> {
        self.post_json("/v0.2.4/multicurrency-wallets/request", request, None)
            .await
    }

    /// Issue a bank account.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn issue_bank_account(
        &self,
        request: &crate::types::IssueBankAccountRequest,
    ) -> Result<crate::types::MulticurrencyWalletResponse> {
        self.post_json("/v0.2.4/multicurrency-wallets/bank-account", request, None)
            .await
    }

    // ── Info Methods ───────────────────────────────────────────────

    /// Get airtime countries.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_airtime_countries(&self) -> Result<Vec<String>> {
        self.get_json_data("/v0.2.4/info/airtime-countries", None)
            .await
    }

    /// Get assets by country code.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_assets(
        &self,
        country_code: &str,
    ) -> Result<serde_json::Value> {
        let path = "/v0.2.4/info/assets";
        let query = format!("countryCode={}", country_code);
        self.get_json_data(path, Some(&query)).await
    }

    /// Get banks by country code.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_banks(&self, country_code: &str) -> Result<Vec<crate::types::BankInfo>> {
        let path = "/v0.2.4/info/country-banks";
        let query = format!("countryCode={}", country_code);
        self.get_json_data(path, Some(&query)).await
    }

    /// Get bank branches.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_bank_branches(
        &self,
        bank_code: &str,
    ) -> Result<Vec<crate::types::BankBranch>> {
        let path = "/v0.2.4/info/bank-branches";
        let query = format!("bankCode={}", bank_code);
        self.get_json_data(path, Some(&query)).await
    }

    /// Get exchange rates.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_exchange_rates(&self) -> Result<serde_json::Value> {
        self.get_json_data("/v0.2.4/info/exchange-rates", None)
            .await
    }

    /// Convert local currency to USD.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn local_to_usd(
        &self,
        currency: &str,
        amount: &str,
    ) -> Result<crate::types::LocalToUsdData> {
        let path = "/v0.2.4/info/local-amount-to-usd";
        let query = format!(
            "originCurrency={}&amountInOriginCurrency={}",
            currency, amount
        );
        self.get_json_data(path, Some(&query)).await
    }

    /// Convert USD to local currency.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn usd_to_local(
        &self,
        currency: &str,
        amount: &str,
    ) -> Result<crate::types::UsdToLocalData> {
        let path = "/v0.2.4/info/usd-amount-in-local";
        let query = format!("destinationCurrency={}&amountInUSD={}", currency, amount);
        self.get_json_data(path, Some(&query)).await
    }

    /// Get mobile money codes.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_mobile_money_codes(&self) -> Result<Vec<crate::types::MobileMoneyCode>> {
        self.get_json_data("/v0.2.4/info/mobile-money-codes", None)
            .await
    }

    // ── Info Methods (extended) ─────────────────────────────────────

    /// Search banks by name in a country.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn search_banks(
        &self,
        country: &str,
        search: &str,
        skip: Option<i32>,
        take: Option<i32>,
    ) -> Result<crate::types::BankSearchResponse> {
        let path = "/v0.2.4/info/bank-search";
        let mut query = format!("country={}&search={}", country, search);
        if let Some(s) = skip {
            query.push_str(&format!("&skip={}", s));
        }
        if let Some(t) = take {
            query.push_str(&format!("&take={}", t));
        }
        self.get_json(path, Some(&query)).await
    }

    /// Get beneficiary validation rules for a country.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_beneficiary_rules(
        &self,
        country_code: &str,
        method: Option<&str>,
    ) -> Result<crate::types::BeneficiaryRulesResponse> {
        let path = format!("/v0.2.4/info/beneficiary-rules/{}", country_code);
        let query = method.map(|m| format!("method={}", m));
        self.get_json(&path, query.as_deref()).await
    }

    /// Get supported identification types.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_identification_types(
        &self,
    ) -> Result<crate::types::IdentificationTypesResponse> {
        self.get_json("/v0.2.4/info/identification-types", None)
            .await
    }

    /// Estimate fees for a transaction.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn estimate_fees(
        &self,
        request: &crate::types::FeeEstimateRequest,
    ) -> Result<crate::types::FeeEstimateResponse> {
        self.post_json("/v0.2.4/info/fee-estimate", request, None)
            .await
    }

    /// Validate a Chimoney voucher code.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn validate_voucher(
        &self,
        code: &str,
    ) -> Result<crate::types::ValidateVoucherResponse> {
        let path = "/v0.2.4/info/communities/verify-code";
        let body = serde_json::json!({ "code": code });
        self.post_json(path, &body, None).await
    }

    /// Search Canadian bill merchants/payees.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn search_merchants(
        &self,
        search: &str,
    ) -> Result<serde_json::Value> {
        let path = "/v0.2.4/info/bill-merchants/ca";
        let body = serde_json::json!({ "search": search });
        self.post_json_data::<_, serde_json::Value>(path, &body, None)
            .await
    }

    /// Get states/regions for a country.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn get_country_states(
        &self,
        country_code: &str,
    ) -> Result<crate::types::CountryStatesResponse> {
        let path = "/v0.2.4/info/country-states-regions";
        let query = format!("countryCode={}", country_code);
        self.get_json(path, Some(&query)).await
    }

    /// Verify bank account.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn verify_bank_account(
        &self,
        country_code: &str,
        bank_code: &str,
        account_number: &str,
    ) -> Result<Vec<crate::types::VerifiedBankAccount>> {
        let path = "/v0.2.4/info/verify-bank-account";
        let body = serde_json::json!({
            "verifyAccountNumbers": [{
                "countryCode": country_code,
                "account_bank": bank_code,
                "account_number": account_number
            }]
        });
        self.post_json_data(path, &body, None).await
    }

    // ── Passport Methods ────────────────────────────────────────────

    /// Manage APort passport (check, create, or resend).
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn manage_passport(
        &self,
        request: &crate::types::PassportRequest,
    ) -> Result<crate::types::PassportResponse> {
        self.post_json("/v0.2.4/passport", request, None).await
    }

    // ── AI Methods ──────────────────────────────────────────────────

    /// Generate an invoice using AI.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::RequestFailed`] if the HTTP request fails,
    /// [`ChimoneyError::MiddlewareError`] if retry middleware fails,
    /// [`ChimoneyError::ApiError`] if the API returns a non-2xx status,
    /// [`ChimoneyError::RateLimited`] if the API returns 429,
    /// or [`ChimoneyError::ParseError`] if the response cannot be parsed.
    pub async fn generate_invoice(
        &self,
        request: &crate::types::GenerateInvoiceRequest,
    ) -> Result<crate::types::GenerateInvoiceResponse> {
        self.post_json("/v0.2.4/ai/invoice/generate", request, None)
            .await
    }

    /// Handle API response.
    async fn handle_response(&self, response: reqwest::Response) -> Result<String> {
        let status = response.status();
        let text = response
            .text()
            .await
            .map_err(ChimoneyError::RequestFailed)?;

        if status.is_success() {
            Ok(text)
        } else {
            let json: serde_json::Value = serde_json::from_str(&text).unwrap_or_else(|_| {
                serde_json::json!({
                    "code": status.as_u16(),
                    "message": text
                })
            });

            let message = json["message"]
                .as_str()
                .unwrap_or("Unknown error")
                .to_string();

            if status.as_u16() == 429 {
                let retry_after = json["retry_after"].as_u64().unwrap_or(60);
                log::warn!("Rate limited, retry after {}s", retry_after);
                return Err(ChimoneyError::RateLimited { retry_after });
            }

            log::warn!("API error {}: {}", status, message);
            Err(ChimoneyError::ApiError {
                status: status.as_u16(),
                message,
            })
        }
    }
}

impl fmt::Debug for ChimoneyClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChimoneyClient")
            .field("base_url", &self.base_url)
            .field("api_key", &"[REDACTED]")
            .finish()
    }
}

/// Builder for configuring `ChimoneyClient`.
pub struct ChimoneyClientBuilder {
    api_key: String,
    base_url: String,
    max_retries: u32,
    timeout_secs: u64,
}

impl ChimoneyClientBuilder {
    /// Set the base URL.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    /// Set the maximum number of retries.
    pub fn max_retries(mut self, n: u32) -> Self {
        self.max_retries = n;
        self
    }

    /// Set the request timeout in seconds.
    pub fn timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    /// Build the client.
    ///
    /// # Errors
    ///
    /// Returns [`ChimoneyError::ApiKeyEmpty`] if the API key is empty.
    pub fn build(self) -> Result<ChimoneyClient> {
        if self.api_key.is_empty() {
            return Err(ChimoneyError::ApiKeyEmpty);
        }

        let client = build_client(self.max_retries, self.timeout_secs)?;

        Ok(ChimoneyClient {
            client,
            api_key: self.api_key,
            base_url: self.base_url,
        })
    }
}

impl fmt::Debug for ChimoneyClientBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChimoneyClientBuilder")
            .field("base_url", &self.base_url)
            .field("api_key", &"[REDACTED]")
            .field("max_retries", &self.max_retries)
            .field("timeout_secs", &self.timeout_secs)
            .finish()
    }
}
