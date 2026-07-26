use serde::{Deserialize, Serialize};

/// An AI agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    /// Unique identifier for the agent.
    pub id: String,
    /// Display name of the agent.
    pub name: String,
    /// Optional description of the agent's purpose.
    #[serde(default)]
    pub description: Option<String>,
    /// Agent status (`"active"`, `"inactive"`, etc.).
    pub status: String,
    /// Optional wallet ID associated with the agent.
    #[serde(default)]
    pub wallet_id: Option<String>,
    /// Optional agent passport data.
    #[serde(default)]
    pub passport: Option<serde_json::Value>,
    /// Optional arbitrary metadata.
    #[serde(default)]
    pub meta: Option<serde_json::Value>,
    /// ISO 8601 timestamp of agent creation.
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Request to create an agent.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAgentRequest {
    /// Agent name.
    pub name: String,
    /// Optional description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

/// Request to update an agent.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAgentRequest {
    /// Agent ID to update.
    pub agent_id: String,
    /// Optional new name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Optional new description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Optional metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}

/// Request to update agent policies.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateAgentPoliciesRequest {
    /// Agent ID to update policies for.
    pub agent_id: String,
    /// Optional limits configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limits: Option<serde_json::Value>,
    /// Optional capabilities configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<serde_json::Value>,
    /// Optional regions configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<serde_json::Value>,
}

/// Request with just an agent ID.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentIdRequest {
    /// The agent ID.
    pub agent_id: String,
}

/// Response from getting or managing an agent API key.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentApiKeyResponse {
    /// The agent ID this key belongs to.
    #[serde(default)]
    pub agent_id: Option<String>,
    /// Visible prefix of the API key for identification.
    #[serde(default)]
    pub api_key_prefix: Option<String>,
    /// ISO 8601 timestamp of key creation.
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Request to manage an agent API key.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ManageAgentApiKeyRequest {
    /// The agent ID.
    pub agent_id: String,
    /// Action to perform: create, rotate, revoke, delete.
    pub action: String,
}

/// Request to get agent transactions.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentTransactionsRequest {
    /// The agent ID.
    pub agent_id: String,
    /// Optional sub-account filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
}

/// Response from getting agent transactions.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentTransactionsResponse {
    /// Response status (`"success"` or `"error"`).
    pub status: String,
    /// Transaction list, if available.
    #[serde(default)]
    pub data: Option<Vec<serde_json::Value>>,
}

/// Response from agent operations (single agent).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentResponse {
    /// Response status (`"success"` or `"error"`).
    pub status: String,
    /// The agent data payload.
    #[serde(default)]
    pub data: Option<Agent>,
}

/// Response from listing agents.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentListResponse {
    /// Response status (`"success"` or `"error"`).
    pub status: String,
    /// List of agents, if available.
    #[serde(default)]
    pub data: Option<Vec<Agent>>,
}

/// Request to fund an agent.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FundAgentRequest {
    /// The agent ID.
    pub agent_id: String,
    /// Amount in USD.
    pub amount_in_usd: f64,
    /// Optional sub-account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_account: Option<String>,
}

/// Agent capabilities, limits, and regions configuration.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentCapabilitiesLimitsResponse {
    /// Response status (`"success"` or `"error"`).
    pub status: String,
    /// Configuration data payload.
    #[serde(default)]
    pub data: Option<AgentCapabilitiesLimitsData>,
}

/// Agent capabilities, limits, and regions data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgentCapabilitiesLimitsData {
    /// Schema or config version identifier.
    #[serde(default)]
    pub version: Option<String>,
    /// List of enabled capabilities.
    #[serde(default)]
    pub capabilities: Option<Vec<serde_json::Value>>,
    /// Spending or usage limits.
    #[serde(default)]
    pub limits: Option<serde_json::Value>,
    /// Allowed geographic regions.
    #[serde(default)]
    pub regions: Option<serde_json::Value>,
}
