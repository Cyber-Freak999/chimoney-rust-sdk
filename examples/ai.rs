use chimoney_rust_sdk::types::*;
use chimoney_rust_sdk::{ChimoneyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ChimoneyClient::new_sandbox("your_api_key")?;

    // Generate an invoice
    let invoice = client
        .generate_invoice(&GenerateInvoiceRequest {
            instruction: "Generate an invoice for $50 USD to john@example.com".to_string(),
        })
        .await?;
    println!("Invoice: {:?}", invoice);

    Ok(())
}
