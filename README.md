# Chimoney Rust SDK

A Rust client library for the [Chimoney API](https://chimoney.readme.io/) — enabling bulk global payments, payouts, and financial transactions.

## Features

- **Full API coverage** — 84 endpoints across all Chimoney API domains
- **Typed request/response structs** — compile-time safety for all API interactions
- **Retry with exponential backoff** — automatic retry for transient failures
- **Sandbox support** — test against the Chimoney sandbox environment
- **Builder pattern** — configurable client with custom timeout, retries, and base URL

## Installation

```bash
cargo add chimoney_rust_sdk
```

## Quick Start

```rust
use chimoney_rust_sdk::ChimoneyClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = ChimoneyClient::new("your_api_key")?;

    // Get supported airtime countries
    let countries = client.get_airtime_countries().await?;
    println!("Supported countries: {:?}", countries);

    // Get exchange rates
    let rates = client.get_exchange_rates().await?;
    println!("Exchange rates: {:?}", rates);

    Ok(())
}
```

## Sandbox

```rust
let client = ChimoneyClient::new_sandbox("your_sandbox_api_key")?;
```

## Configuration

```rust
let client = ChimoneyClient::builder("your_api_key")
    .base_url("https://api.chimoney.io")
    .max_retries(5)
    .timeout(60)
    .build()?;
```

## API Reference

- [Chimoney API Documentation](https://chimoney.readme.io/)
- [API Reference](https://chimoney.readme.io/reference/introduction)
- [docs.rs](https://docs.rs/chimoney_rust_sdk)

## Supported Endpoints

| Domain | Endpoints |
|--------|-----------|
| Account | transactions, transfer, public profile, wallet address, claim reward |
| Agent | CRUD, fund, transactions, API keys, policies, preview transfer, capabilities |
| Beneficiary | create, list, delete |
| Info | bank search, beneficiary rules, ID types, fee estimate, merchants, country states |
| Multicurrency Wallet | create, list, get, update, transfer, fund, quote, issue bank account |
| Payment | initiate, verify, simulate (staging) |
| Payout | airtime, bank, mobile money, gift card, crypto, Interac, SPEI, Interledger, bills |
| Passport | check, create, resend |
| Redeem | airtime, gift card, mobile money, chimoney, any |
| SubAccount | CRUD, community create/update/members, KYC link |

## License

MIT
