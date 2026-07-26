use chimoney_rust_sdk::types::*;
use chimoney_rust_sdk::{ChimoneyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ChimoneyClient::new_sandbox("your_api_key")?;

    // Initiate a payment
    let payment = client
        .initiate_payment(&PaymentRequest {
            email: "payer@example.com".to_string(),
            amount: 50.0,
            redirect_url: "https://example.com/success".to_string(),
            sub_account: None,
        })
        .await?;
    println!("Payment: {:?}", payment);

    // Verify a payment by issue ID
    let verification = client.verify_payment("issue_id_here").await?;
    println!("Verification: {:?}", verification);

    Ok(())
}
