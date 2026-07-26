use chimoney_rust_sdk::types::*;
use chimoney_rust_sdk::{ChimoneyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ChimoneyClient::new_sandbox("your_api_key")?;

    // Bank payout
    let bank_payout = client
        .payout_bank(&BankPayoutRequest {
            base: PayoutRequest {
                sub_account: None,
                turn_off_notification: None,
            },
            transfers: vec![BankTransfer {
                bank_code: "044".to_string(),
                account_number: "1234567890".to_string(),
                amount: 5000.0,
                currency: "NGN".to_string(),
                country_code: "NG".to_string(),
                beneficiary_name: Some("John Doe".to_string()),
            }],
        })
        .await?;
    println!("Bank payout: {:?}", bank_payout);

    // Airtime payout
    let airtime_payout = client
        .payout_airtime(&AirtimePayoutRequest {
            base: PayoutRequest {
                sub_account: None,
                turn_off_notification: None,
            },
            transfers: vec![AirtimeTransfer {
                phone_number: "+2348012345678".to_string(),
                amount: 5.0,
                country_code: "NG".to_string(),
            }],
        })
        .await?;
    println!("Airtime payout: {:?}", airtime_payout);

    // Chimoney payout
    let chimoney_payout = client
        .payout_chimoney(&ChimoneyPayoutRequest {
            base: PayoutRequest {
                sub_account: None,
                turn_off_notification: None,
            },
            transfers: vec![ChimoneyTransfer {
                receiver: "recipient@example.com".to_string(),
                value_in_usd: 10.0,
            }],
        })
        .await?;
    println!("Chimoney payout: {:?}", chimoney_payout);

    // Check payout status
    let status = client.check_payout_status("chi_ref_here").await?;
    println!("Payout status: {:?}", status);

    Ok(())
}
