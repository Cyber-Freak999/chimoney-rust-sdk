use chimoney_rust_sdk::types::*;
use chimoney_rust_sdk::{ChimoneyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ChimoneyClient::new_sandbox("your_api_key")?;

    // List all beneficiaries
    let beneficiaries = client.get_beneficiaries().await?;
    println!("Beneficiaries: {:?}", beneficiaries);

    // Create a bank beneficiary
    let beneficiary = client
        .create_bank_beneficiary(&CreateBankBeneficiaryRequest {
            account_number: "1234567890".to_string(),
            bank_code: "044".to_string(),
            country_code: "NG".to_string(),
            name: "John Doe".to_string(),
            currency: "NGN".to_string(),
        })
        .await?;
    println!("Created beneficiary: {:?}", beneficiary);

    // Preview a transfer
    let preview = client
        .preview_transfer(&PreviewTransferRequest {
            beneficiary_id: "ben_123".to_string(),
            amount: 50.0,
            currency: "USD".to_string(),
        })
        .await?;
    println!("Preview: {:?}", preview);

    Ok(())
}
