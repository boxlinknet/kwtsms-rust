use serde::{Deserialize, Serialize};

/// A phone number that failed local pre-validation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InvalidEntry {
    pub input: String,
    pub error: String,
}

/// Result of `verify()`: credential check + balance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifyResult {
    pub ok: bool,
    pub balance: Option<f64>,
    pub purchased: Option<f64>,
    pub error: Option<String>,
}

/// Result of a single `send()` call (up to 200 numbers).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendResult {
    pub result: String,
    #[serde(default, rename = "msg-id")]
    pub msg_id: Option<String>,
    #[serde(default)]
    pub numbers: Option<u32>,
    #[serde(default, rename = "points-charged")]
    pub points_charged: Option<u32>,
    #[serde(default, rename = "balance-after")]
    pub balance_after: Option<f64>,
    #[serde(default, rename = "unix-timestamp")]
    pub unix_timestamp: Option<i64>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default)]
    pub invalid: Vec<InvalidEntry>,
}

/// A single batch error in a bulk send.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchError {
    pub batch: u32,
    pub code: String,
    pub description: String,
    #[serde(default)]
    pub action: Option<String>,
}

/// Result of a bulk send (>200 numbers, auto-batched).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkSendResult {
    /// "OK", "PARTIAL", or "ERROR"
    pub result: String,
    pub bulk: bool,
    pub batches: u32,
    pub numbers: u32,
    #[serde(rename = "points-charged")]
    pub points_charged: u32,
    #[serde(rename = "balance-after")]
    pub balance_after: Option<f64>,
    #[serde(rename = "msg-ids")]
    pub msg_ids: Vec<String>,
    #[serde(default)]
    pub errors: Vec<BatchError>,
    #[serde(default)]
    pub invalid: Vec<InvalidEntry>,
}

/// Result of `validate()`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidateResult {
    pub ok: Vec<String>,
    pub er: Vec<String>,
    pub nr: Vec<String>,
    pub raw: Option<serde_json::Value>,
    pub error: Option<String>,
    #[serde(default)]
    pub rejected: Vec<InvalidEntry>,
}

/// Result of `senderids()`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenderIdResult {
    pub result: String,
    #[serde(default)]
    pub senderids: Vec<String>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub action: Option<String>,
}

/// Result of `coverage()`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoverageResult {
    pub result: String,
    #[serde(default)]
    pub prefixes: Vec<String>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub action: Option<String>,
}

/// Result of `status()`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResult {
    pub result: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub action: Option<String>,
}

/// A single DLR report entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlrEntry {
    #[serde(default, alias = "Number")]
    pub number: Option<String>,
    #[serde(default, alias = "Status")]
    pub status: Option<String>,
}

/// Result of `dlr()`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DlrResult {
    pub result: String,
    #[serde(default)]
    pub report: Vec<DlrEntry>,
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub action: Option<String>,
}
