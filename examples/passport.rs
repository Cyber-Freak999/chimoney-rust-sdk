use chimoney_rust_sdk::types::*;
use chimoney_rust_sdk::{ChimoneyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ChimoneyClient::new_sandbox("your_api_key")?;

    // Check passport status
    let status = client
        .manage_passport(&PassportRequest {
            action: PassportAction::Check,
            account_id: None,
        })
        .await?;
    println!("Passport status: {:?}", status);

    // Create passport
    let created = client
        .manage_passport(&PassportRequest {
            action: PassportAction::Create,
            account_id: Some("user_123".to_string()),
        })
        .await?;
    println!("Created passport: {:?}", created);

    // Resend passport email
    let resent = client
        .manage_passport(&PassportRequest {
            action: PassportAction::Resend,
            account_id: Some("user_123".to_string()),
        })
        .await?;
    println!("Resent passport: {:?}", resent);

    Ok(())
}
