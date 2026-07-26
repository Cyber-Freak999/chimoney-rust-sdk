use chimoney_rust_sdk::types::*;
use chimoney_rust_sdk::{ChimoneyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ChimoneyClient::new_sandbox("your_api_key")?;

    // Create a community
    let community = client
        .create_community(&CreateCommunityRequest {
            id: "user_123".to_string(),
            community: Community {
                community_id: 456,
                name: "My Community".to_string(),
                membership_id: "mem_789".to_string(),
                member_name: "John Doe".to_string(),
                community_type: "free".to_string(),
                voucher_code: None,
            },
        })
        .await?;
    println!("Created community: {:?}", community);

    // Update a community
    let updated = client
        .update_community(&UpdateCommunityRequest {
            id: "user_123".to_string(),
            community_id: 456,
            membership_id: Some("mem_789".to_string()),
            member_name: Some("Jane Doe".to_string()),
        })
        .await?;
    println!("Updated community: {:?}", updated);

    // Get community members
    let members = client
        .get_community_members(456, Some(10), None, None)
        .await?;
    println!("Members: {:?}", members);

    // Get KYC link
    let kyc = client
        .get_kyc_link("sub_123", Some("https://example.com/done"))
        .await?;
    println!("KYC link: {:?}", kyc);

    Ok(())
}
