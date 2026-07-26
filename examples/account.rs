use chimoney_rust_sdk::types::*;
use chimoney_rust_sdk::{ChimoneyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ChimoneyClient::new_sandbox("your_api_key")?;

    // Transfer between accounts
    let transfer = client
        .transfer(&TransferRequest {
            receiver: "recipient@example.com".to_string(),
            value_in_usd: 10.0,
            sub_account: None,
        })
        .await?;
    println!("Transfer: {:?}", transfer);

    // Initiate a Chimoney transaction
    let initiate = client
        .initiate_chimoney(&InitiateChimoneyRequest {
            receiver: "user@example.com".to_string(),
            value_in_usd: 25.0,
            sub_account: None,
            turn_off_notification: None,
        })
        .await?;
    println!("Initiated: {:?}", initiate);

    // Get transactions for an account
    let transactions = client.get_transactions("account_id_here").await?;
    println!("Transactions: {}", transactions.len());

    // Issue an Interledger wallet address
    let wallet = client
        .issue_wallet_address(&IssueWalletAddressRequest {
            user_id: "user_123".to_string(),
            ilp_username: "wallet".to_string(),
        })
        .await?;
    println!("Wallet: {:?}", wallet);

    // Claim a community reward
    let reward = client
        .claim_reward(&ClaimRewardRequest {
            user_id: "user_123".to_string(),
            community: ClaimRewardCommunity {
                community_id: "456".to_string(),
                membership_id: None,
                voucher_code: None,
                name: Some("My Community".to_string()),
                member_name: Some("John Doe".to_string()),
                community_type: Some("free".to_string()),
                expiry_date: None,
            },
        })
        .await?;
    println!("Reward: {:?}", reward);

    Ok(())
}
