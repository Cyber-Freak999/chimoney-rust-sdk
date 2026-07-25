use serde::{Deserialize, Serialize};

/// Request to generate an AI invoice.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateInvoiceRequest {
    /// Instruction to generate an invoice.
    pub instruction: String,
}

/// Invoice JSON data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceJson {
    #[serde(default)]
    pub invoice_date: Option<String>,
    #[serde(default)]
    pub invoice_number: Option<f64>,
    #[serde(default)]
    pub due_date: Option<String>,
}

/// Generated invoice data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceData {
    #[serde(default, rename = "downloadURL")]
    pub download_url: Option<String>,
    #[serde(default, rename = "chimoneyPaymentRequestCreateURL")]
    pub chimoney_payment_request_create_url: Option<String>,
    #[serde(default, rename = "chimoneyAPIEndpoint")]
    pub chimoney_api_endpoint: Option<String>,
    #[serde(default)]
    pub json: Option<InvoiceJson>,
}

/// Response from invoice generation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateInvoiceResponse {
    pub status: String,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub data: Option<InvoiceData>,
}
