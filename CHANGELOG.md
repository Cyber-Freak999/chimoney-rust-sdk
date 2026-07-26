# Changelog

## [1.4.1] - 2026-07-26

### Added
- Comprehensive field-level rustdoc comments across all 12 type modules
- Documented ~267 public fields in ~94 structs/enums
- Crate-level documentation with feature summary and usage examples
- `pub mod` and `pub use` re-exports in `lib.rs` now documented

### Metrics
- docs.rs coverage: 59% → ~100% (all public items documented)

## [1.4.0] - 2026-07-26

### Added
- 3 new endpoint methods:
  - `simulate_interac_funding` — Simulate Interac funding (sandbox)
  - `local_amount_in_usd` — Convert local currency amount to USD (newer endpoint)
  - `verify_bank_account_number` — Verify bank account numbers (legacy v0.2)
- 7 domain-specific example files:
  - `examples/beneficiary.rs` — get_beneficiaries, create_bank_beneficiary, preview_transfer
  - `examples/redeem.rs` — redeem_airtime, redeem_chimoney, redeem_giftcard, redeem_mobile_money, redeem_any
  - `examples/subaccount.rs` — create, update, delete, get, list sub-accounts
  - `examples/community.rs` — create_community, update_community, get_community_members, get_kyc_link
  - `examples/info.rs` — all 16 info endpoints
  - `examples/passport.rs` — manage_passport (check, create, resend)
  - `examples/ai.rs` — generate_invoice

### Fixed
- `payout_interledger` path updated to `/payouts/interledger-wallet-address`
- `get_transfer_quote` path updated to `/multicurrency-wallets/transfer/quote`
- `update_multicurrency_wallet` method changed from PATCH to POST
- `get_beneficiary_rules` path parameter renamed to camelCase
- `verify_bank_account` moved to legacy v0.2 endpoint

### Metrics
- Endpoints: 83 → 86 (100% API coverage)
- Examples: 9 → 16 (all 13 domains covered)

## [1.3.0] - 2026-07-26

### Added
- Serde tests for 17 response/request wrapper types (148 total tests)
- 6 domain-specific examples: account, payment, payout, agent, wallet, multicurrency

### Changed
- Improved test coverage from 136 to 148 tests

## [1.2.0] - 2026-07-26

### Added
- Typed response structs for 13 methods (previously returned `serde_json::Value`)
- Logging for request/response/error events via `log` crate

### Changed
- Bumped MSRV from 1.65 to 1.85 (required by transitive dependency `idna_adapter`)
- Upgraded edition from 2021 to 2024
- Inlined format args (clippy cleanup)

## [1.1.0] - 2026-07-26

### Added
- GitHub Actions CI/CD workflow (test, clippy, fmt, audit, MSRV)
- `# Errors` doc sections for all public methods
- `Debug` implementation for `ChimoneyClient` and `ChimoneyClientBuilder`

### Changed
- Refactored HTTP helpers: 4 methods → 1 unified `request()` method
- Added generic response parsing helpers (`post_json`, `get_json`, etc.)
- Upgraded `reqwest` 0.11 → 0.12
- Upgraded `thiserror` 1 → 2
- Upgraded `reqwest-retry` 0.2 → 0.6

### Removed
- Unused `dotenv` dependency

## [1.0.0] - 2026-07-25

### Added
- **Full API coverage** — 84 endpoints across all Chimoney API domains
- Info endpoints: bank search, beneficiary rules, ID types, fee estimate, voucher validate, merchants, country states
- Payout extras: Interac, SPEI, process unpaid, Canadian bills
- SubAccount community: create, update, members, KYC link
- Account: issue wallet address, claim reward
- Redeem: redeem any
- Passport: manage APort passport
- AI: generate invoice
- Agent: capabilities-limits
- Payment: simulate funding (staging)
- 125 tests passing
- Domain graph: 362 nodes, 362 edges

## [0.3.0] - 2026-07-24

### Added
- Agent endpoints: create, update, list, fund, transactions, manage API keys, policies, preview transfer
- Beneficiary endpoints: create, list, delete
- Multicurrency Wallet endpoints: create, list, get, update, transfer, fund, quote, issue bank account

## [0.2.0] - 2026-07-23

### Added
- `ChimoneyClient` struct with builder pattern
- Custom `ChimoneyError` enum with `thiserror`
- Typed request/response structs for all endpoints
- Retry middleware with exponential backoff
- Connection pooling via `reqwest-middleware`
- Sandbox support via `ChimoneyClient::new_sandbox()`
- Documentation and examples

### Changed
- All functions now return `Result<T, ChimoneyError>` instead of `Result<String, Box<dyn Error>>`
- API methods are now on `ChimoneyClient` struct instead of free functions
- Responses are now deserialized to typed structs

### Removed
- `APIClient` struct
- Free functions for all endpoints
- `dotenv` dependency

## [0.1.0] - Initial release
