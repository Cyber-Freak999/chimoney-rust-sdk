# Chimoney Rust SDK

A Rust client library for the [Chimoney API](https://chimoney.readme.io/) — enabling bulk global payments, payouts, and financial transactions.

## Features

- **Full API coverage** — 86 endpoints across all Chimoney API domains
- **Typed request/response structs** — compile-time safety for all API interactions
- **Retry with exponential backoff** — automatic retry for transient failures
- **Sandbox support** — test against the Chimoney sandbox environment
- **Builder pattern** — configurable client with custom timeout, retries, and base URL
- **16 domain-specific examples** — ready-to-run code for every API domain

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

## Examples

Run any example with:

```bash
cargo run --example <name>
```

| Example | Domain | Endpoints Covered |
|---------|--------|-------------------|
| `account` | Account | transfer, initiate_chimoney, transactions, wallet address, claim reward |
| `agent` | Agent | create, fund, list, capabilities |
| `ai` | AI | generate_invoice |
| `beneficiary` | Beneficiary | list, create, preview transfer |
| `community` | Community | create, update, members, KYC link |
| `info` | Info | all 16 endpoints (banks, assets, exchange rates, fees, etc.) |
| `multicurrency` | Multicurrency Wallet | create, list, transfer, quote, issue bank account |
| `passport` | Passport | check, create, resend |
| `payment` | Payment | initiate, verify |
| `payout` | Payout | bank, airtime, chimoney, check status |
| `redeem` | Redeem | airtime, chimoney, gift card, mobile money, any |
| `subaccount` | SubAccount | create, update, delete, get, list |
| `wallet` | Wallet | list, lookup, transfer |

## Supported Endpoints

| Domain | Endpoints | Count |
|--------|-----------|-------|
| Account | transactions, transfer, initiate chimoney, delete unpaid, wallet address, claim reward | 9 |
| Agent | CRUD, fund, transactions, API keys, policies, suspend, reactivate, capabilities | 12 |
| AI | generate invoice | 1 |
| Beneficiary | list, create bank, preview transfer | 3 |
| Community | create, update, members, KYC link | 4 |
| Info | airtime countries, assets, banks, branches, exchange rates, currency conversion, mobile money, bank search, beneficiary rules, ID types, fee estimate, voucher validate, merchants, country states, verify bank | 16 |
| Multicurrency Wallet | create, update, get, list, transfer, quote, issue wallet, issue bank account | 8 |
| Passport | check, create, resend | 1 |
| Payment | initiate, verify, simulate, simulate funding, simulate Interac funding | 5 |
| Payout | bank, airtime, chimoney, mobile money, gift card, interledger, wallet, Interac, SPEI, bills CA, process unpaid, check status | 12 |
| Redeem | airtime, chimoney, gift card, mobile money, any | 5 |
| SubAccount | create, update, delete, get, list | 5 |
| Wallet | list, lookup, transfer | 3 |
| **Total** | | **86** |

## API Reference

- [Chimoney API Documentation](https://chimoney.readme.io/)
- [API Reference](https://chimoney.readme.io/reference/introduction)
- [docs.rs](https://docs.rs/chimoney_rust_sdk)

## License

MIT
