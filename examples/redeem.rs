use chimoney_rust_sdk::types::*;
use chimoney_rust_sdk::{ChimoneyClient, Result};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    let client = ChimoneyClient::new_sandbox("your_api_key")?;

    // Redeem airtime
    let airtime = client
        .redeem_airtime(&RedeemAirtimeRequest {
            base: RedeemRequest {
                sub_account: "sub_123".to_string(),
                chi_ref: None,
                turn_off_notification: None,
            },
            country_to_send: "NG".to_string(),
            phone_number: "+2348012345678".to_string(),
            test: None,
        })
        .await?;
    println!("Redeem airtime: {:?}", airtime);

    // Redeem chimoney
    let chimoney = client
        .redeem_chimoney(&RedeemChimoneyRequest {
            base: RedeemRequest {
                sub_account: "sub_123".to_string(),
                chi_ref: None,
                turn_off_notification: None,
            },
            chimoneys: HashMap::from([("chi_ref_123".to_string(), "10.0".to_string())]),
        })
        .await?;
    println!("Redeem chimoney: {:?}", chimoney);

    // Redeem gift card
    let giftcard = client
        .redeem_giftcard(&RedeemGiftCardRequest {
            base: RedeemRequest {
                sub_account: "sub_123".to_string(),
                chi_ref: None,
                turn_off_notification: None,
            },
            redeem_options: HashMap::from([("card_code".to_string(), "ABC123".to_string())]),
        })
        .await?;
    println!("Redeem gift card: {:?}", giftcard);

    // Redeem mobile money
    let mobile = client
        .redeem_mobile_money(&RedeemMobileMoneyRequest {
            base: RedeemRequest {
                sub_account: "sub_123".to_string(),
                chi_ref: None,
                turn_off_notification: None,
            },
            redeem_options: HashMap::from([("phone".to_string(), "+2348012345678".to_string())]),
        })
        .await?;
    println!("Redeem mobile money: {:?}", mobile);

    // Redeem any
    let any = client
        .redeem_any(&RedeemAnyRequest {
            sub_account: Some("sub_123".to_string()),
            chi_ref: "chi_ref_456".to_string(),
            meta: None,
            redeem_data: serde_json::json!({"type": "airtime", "phone": "+2348012345678"}),
        })
        .await?;
    println!("Redeem any: {:?}", any);

    Ok(())
}
