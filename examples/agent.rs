use chimoney_rust_sdk::types::*;
use chimoney_rust_sdk::{ChimoneyClient, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = ChimoneyClient::new_sandbox("your_api_key")?;

    // Create a new agent
    let agent = client
        .create_agent(&CreateAgentRequest {
            name: "Customer Support Bot".to_string(),
            description: Some("Handles customer inquiries".to_string()),
            meta: None,
        })
        .await?;
    println!("Created agent: {:?}", agent);

    // Fund an agent wallet
    let funded = client
        .fund_agent(&FundAgentRequest {
            agent_id: agent.data.as_ref().unwrap().id.clone(),
            amount_in_usd: 100.0,
            sub_account: None,
        })
        .await?;
    println!("Funded agent: {:?}", funded);

    // List all agents
    let agents = client.list_agents().await?;
    let count = agents.data.as_ref().map_or(0, |v| v.len());
    println!("Total agents: {}", count);

    // Get agent capabilities and limits
    let caps = client.get_agent_capabilities_limits().await?;
    println!("Capabilities: {:?}", caps);

    Ok(())
}
