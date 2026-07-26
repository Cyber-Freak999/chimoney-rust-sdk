use chimoney_rust_sdk::{ChimoneyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ChimoneyClient::new_sandbox("your_api_key")?;

    // Get airtime countries
    let countries = client.get_airtime_countries().await?;
    println!("Airtime countries: {:?}", countries);

    // Get assets for a country
    let assets = client.get_assets("NG").await?;
    println!("Assets: {:?}", assets);

    // Get banks for a country
    let banks = client.get_banks("NG").await?;
    println!("Banks: {:?}", banks);

    // Get bank branches
    let branches = client.get_bank_branches("044").await?;
    println!("Branches: {:?}", branches);

    // Get exchange rates
    let rates = client.get_exchange_rates().await?;
    println!("Exchange rates: {:?}", rates);

    // Convert local to USD
    let usd = client.local_to_usd("NGN", "1000").await?;
    println!("Local to USD: {:?}", usd);

    // Convert USD to local
    let local = client.usd_to_local("NGN", "10").await?;
    println!("USD to local: {:?}", local);

    // Get mobile money codes
    let codes = client.get_mobile_money_codes().await?;
    println!("Mobile money codes: {:?}", codes);

    // Search banks
    let results = client
        .search_banks("NG", "access", Some(0), Some(10))
        .await?;
    println!("Search results: {:?}", results);

    // Get beneficiary rules
    let rules = client.get_beneficiary_rules("NG", None).await?;
    println!("Beneficiary rules: {:?}", rules);

    // Get identification types
    let id_types = client.get_identification_types().await?;
    println!("ID types: {:?}", id_types);

    // Estimate fees
    let fees = client
        .estimate_fees(&chimoney_rust_sdk::types::FeeEstimateRequest {
            amount: 100.0,
            currency: Some("USD".to_string()),
            rail: Some("bank".to_string()),
            direction: Some("outbound".to_string()),
        })
        .await?;
    println!("Fee estimate: {:?}", fees);

    // Validate voucher
    let voucher = client.validate_voucher("ABC123").await?;
    println!("Voucher: {:?}", voucher);

    // Search merchants
    let merchants = client.search_merchants("electric").await?;
    println!("Merchants: {:?}", merchants);

    // Get country states
    let states = client.get_country_states("NG").await?;
    println!("States: {:?}", states);

    // Verify bank account
    let verified = client
        .verify_bank_account("NG", "044", "1234567890")
        .await?;
    println!("Verified: {:?}", verified);

    // Newer local to USD endpoint
    let usd2 = client.local_amount_in_usd("NGN", "1000").await?;
    println!("Local amount in USD: {:?}", usd2);

    Ok(())
}
