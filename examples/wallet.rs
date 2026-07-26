use chimoney_rust_sdk::types::*;
use chimoney_rust_sdk::{ChimoneyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ChimoneyClient::new_sandbox("your_api_key")?;

    // List wallets for a sub-account
    let wallets = client.list_wallets("sub_account_id").await?;
    println!("Wallets: {:?}", wallets);

    // Lookup a wallet by ID
    let lookup = client
        .lookup_wallet(&WalletLookupRequest {
            wallet_id: "wallet_123".to_string(),
            sub_account: "sub_account_id".to_string(),
        })
        .await?;
    println!("Lookup: {:?}", lookup);

    // Transfer between wallets
    let transfer = client
        .transfer_between_wallets(&WalletTransferRequest {
            wallet: "wallet_from_123".to_string(),
            value_in_usd: 25.0,
            sub_account: "sub_account_id".to_string(),
            receiver: "wallet_to_456".to_string(),
        })
        .await?;
    println!("Transfer: {:?}", transfer);

    Ok(())
}
