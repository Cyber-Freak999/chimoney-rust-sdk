use chimoney_rust_sdk::types::*;
use chimoney_rust_sdk::{ChimoneyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ChimoneyClient::new_sandbox("your_api_key")?;

    // Create a multicurrency wallet
    let wallet = client
        .create_multicurrency_wallet(&CreateMulticurrencyWalletRequest {
            sub_account: "sub_account_id".to_string(),
            currency: "USD".to_string(),
            meta: None,
        })
        .await?;
    println!("Created wallet: {:?}", wallet);

    // List all multicurrency wallets
    let wallets = client.list_multicurrency_wallets().await?;
    println!("All wallets: {:?}", wallets);

    // Get a transfer quote
    let quote = client
        .get_transfer_quote(&TransferQuoteRequest {
            from_wallet: "wallet_from_123".to_string(),
            to_wallet: "wallet_to_456".to_string(),
            amount: 100.0,
            from_currency: "USD".to_string(),
            to_currency: "NGN".to_string(),
        })
        .await?;
    println!("Quote: {:?}", quote);

    // Transfer between multicurrency wallets
    let transfer = client
        .transfer_multicurrency(&MulticurrencyTransferRequest {
            from_wallet: "wallet_from_123".to_string(),
            recipient: "wallet_to_456".to_string(),
            amount: 50.0,
            from_currency: "USD".to_string(),
            to_currency: "NGN".to_string(),
            note: Some("Cross-border payment".to_string()),
        })
        .await?;
    println!("Transfer: {:?}", transfer);

    // Issue a bank account
    let bank_account = client
        .issue_bank_account(&IssueBankAccountRequest {
            sub_account: "sub_account_id".to_string(),
            country_code: "US".to_string(),
            bank_code: "021000021".to_string(),
            account_number: "123456789".to_string(),
        })
        .await?;
    println!("Bank account: {:?}", bank_account);

    Ok(())
}
