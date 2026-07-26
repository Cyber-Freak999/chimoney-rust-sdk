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
    /// The date the invoice was created.
    #[serde(default)]
    pub invoice_date: Option<String>,
    /// Unique identifier for the invoice.
    #[serde(default)]
    pub invoice_number: Option<f64>,
    /// Date by which payment is due.
    #[serde(default)]
    pub due_date: Option<String>,
}

/// Generated invoice data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvoiceData {
    /// URL to download the generated invoice PDF.
    #[serde(default, rename = "downloadURL")]
    pub download_url: Option<String>,
    /// URL to create a Chimoney payment request for this invoice.
    #[serde(default, rename = "chimoneyPaymentRequestCreateURL")]
    pub chimoney_payment_request_create_url: Option<String>,
    /// Chimoney API endpoint for this invoice.
    #[serde(default, rename = "chimoneyAPIEndpoint")]
    pub chimoney_api_endpoint: Option<String>,
    /// Parsed invoice JSON data containing date and number fields.
    #[serde(default)]
    pub json: Option<InvoiceJson>,
}

/// Response from invoice generation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateInvoiceResponse {
    /// Status of the invoice generation request.
    pub status: String,
    /// Optional message providing additional details about the response.
    #[serde(default)]
    pub message: Option<String>,
    /// Generated invoice data including download URL and payment details.
    #[serde(default)]
    pub data: Option<InvoiceData>,
}
