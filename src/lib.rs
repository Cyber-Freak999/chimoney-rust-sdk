//! Chimoney Rust SDK
//!
//! A Rust client library for the [Chimoney API](https://chimoney.readme.io/) —
//! enabling bulk global payments, payouts, and financial transactions.
//!
//! # Features
//!
//! - **Full API coverage** — 86 endpoints across all Chimoney API domains
//! - **Typed request/response structs** — compile-time safety for all API interactions
//! - **Retry with exponential backoff** — automatic retry for transient failures
//! - **Sandbox support** — test against the Chimoney sandbox environment
//! - **Builder pattern** — configurable client with custom timeout, retries, and base URL
//!
//! # Example
//!
//! ```no_run
//! use chimoney_rust_sdk::{ChimoneyClient, error::Result};
//!
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let client = ChimoneyClient::new("your_api_key")?;
//!
//! // Get supported airtime countries
//! let countries = client.get_airtime_countries().await?;
//! println!("Countries: {:?}", countries);
//!
//! // Get exchange rates
//! let rates = client.get_exchange_rates().await?;
//! println!("Rates: {:?}", rates);
//!
//! # Ok(())
//! # }
//! ```

/// HTTP client for the Chimoney API.
///
/// Provides typed methods for all 86 Chimoney API endpoints across
/// Account, Payment, Payout, Agent, Beneficiary, Redeem, SubAccount,
/// Community, Wallet, Multicurrency Wallet, Info, Passport, and AI domains.
pub mod client;

/// Error types for the Chimoney SDK.
///
/// Contains [`ChimoneyError`](error::ChimoneyError) enum with variants for
/// API errors, network failures, parsing errors, and rate limiting.
pub mod error;

/// HTTP middleware configuration.
///
/// Handles retry logic with exponential backoff, timeout configuration,
/// and client construction with proper headers.
pub mod middleware;

/// Request and response types for all Chimoney API endpoints.
///
/// Each API domain has its own module:
/// - [`account`](types::account) — Transfer, initiate chimoney, transactions
/// - [`agent`](types::agent) — AI agent management
/// - [`ai`](types::ai) — Invoice generation
/// - [`beneficiary`](types::beneficiary) — Bank beneficiary management
/// - [`info`](types::info) — Banks, assets, exchange rates, fee estimates
/// - [`multicurrency_wallet`](types::multicurrency_wallet) — Multi-currency wallet operations
/// - [`passport`](types::passport) — APort passport management
/// - [`payment`](types::payment) — Payment initiation and verification
/// - [`payout`](types::payout) — Payouts via bank, airtime, mobile money, gift card, etc.
/// - [`redeem`](types::redeem) — Redeem airtime, gift cards, mobile money
/// - [`subaccount`](types::subaccount) — Sub-account and community management
/// - [`wallet`](types::wallet) — Wallet listing and transfers
pub mod types;

/// Re-exported [`ChimoneyClient`](client::ChimoneyClient) for convenient access.
pub use client::ChimoneyClient;

/// Re-exported [`ChimoneyClientBuilder`](client::ChimoneyClientBuilder) for convenient access.
pub use client::ChimoneyClientBuilder;

/// Re-exported [`ChimoneyError`](error::ChimoneyError) for convenient access.
pub use error::ChimoneyError;

/// Re-exported [`Result`](error::Result) type alias for convenient access.
pub use error::Result;
