use chimoney_rust_sdk::types::*;
use chimoney_rust_sdk::{ChimoneyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ChimoneyClient::new_sandbox("your_api_key")?;

    // Create a sub-account
    let sub_account = client
        .create_sub_account(&CreateSubAccountRequest {
            name: "My Sub Account".to_string(),
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            email: "john@example.com".to_string(),
            phone_number: "+2348012345678".to_string(),
        })
        .await?;
    println!("Created sub-account: {:?}", sub_account);

    // List all sub-accounts
    let sub_accounts = client.list_sub_accounts().await?;
    println!("Sub-accounts: {:?}", sub_accounts);

    // Get a specific sub-account
    let sub = client.get_sub_account("sub_123").await?;
    println!("Sub-account: {:?}", sub);

    // Update a sub-account
    let updated = client
        .update_sub_account(&UpdateSubAccountRequest {
            id: "sub_123".to_string(),
            first_name: Some("Jane".to_string()),
            last_name: None,
            phone_number: None,
            meta: None,
        })
        .await?;
    println!("Updated: {:?}", updated);

    // Delete a sub-account
    let deleted = client.delete_sub_account("sub_123").await?;
    println!("Deleted: {:?}", deleted);

    Ok(())
}
