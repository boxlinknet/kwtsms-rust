use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use serde_json::{json, Value};

use crate::env::load_env_file;
use crate::errors::{KwtSmsError, API_ERRORS};
use crate::message::clean_message;
use crate::phone::validate_phone_input;
use crate::request::api_request;
use crate::types::*;

const MAX_BATCH_SIZE: usize = 200;
const BATCH_DELAY_MS: u64 = 500;
const ERR013_BACKOFF_SECS: &[u64] = &[30, 60, 120];

/// kwtSMS API client.
///
/// Thread-safe: can be shared across threads via `Arc<KwtSms>`.
///
/// # Example
/// ```no_run
/// use kwtsms::KwtSms;
///
/// let sms = KwtSms::new("username", "password", None, false, None).unwrap();
/// let result = sms.verify();
/// ```
pub struct KwtSms {
    username: String,
    password: String,
    sender_id: String,
    test_mode: bool,
    log_file: String,
    cached_balance: Arc<Mutex<Option<f64>>>,
    cached_purchased: Arc<Mutex<Option<f64>>>,
}

impl KwtSms {
    /// Create a new kwtSMS client.
    ///
    /// # Arguments
    /// - `username`: API username (not your account phone number)
    /// - `password`: API password
    /// - `sender_id`: Sender ID (default: "KWT-SMS"). Case sensitive.
    /// - `test_mode`: If true, messages are queued but not delivered, no credits consumed.
    /// - `log_file`: Path to JSONL log file (None or empty string disables logging).
    pub fn new(
        username: &str,
        password: &str,
        sender_id: Option<&str>,
        test_mode: bool,
        log_file: Option<&str>,
    ) -> Result<Self, KwtSmsError> {
        let username = username.trim().to_string();
        let password = password.trim().to_string();

        if username.is_empty() {
            return Err(KwtSmsError::InvalidInput(
                "Username is required".to_string(),
            ));
        }
        if password.is_empty() {
            return Err(KwtSmsError::InvalidInput(
                "Password is required".to_string(),
            ));
        }

        Ok(KwtSms {
            username,
            password,
            sender_id: sender_id.unwrap_or("KWT-SMS").to_string(),
            test_mode,
            log_file: log_file.unwrap_or("kwtsms.log").to_string(),
            cached_balance: Arc::new(Mutex::new(None)),
            cached_purchased: Arc::new(Mutex::new(None)),
        })
    }

    /// Create a client from environment variables and/or a `.env` file.
    ///
    /// Reads (in priority order):
    /// 1. Environment variables
    /// 2. `.env` file values
    /// 3. Defaults
    ///
    /// Required: `KWTSMS_USERNAME`, `KWTSMS_PASSWORD`
    /// Optional: `KWTSMS_SENDER_ID` (default "KWT-SMS"), `KWTSMS_TEST_MODE` (default "0"),
    ///           `KWTSMS_LOG_FILE` (default "kwtsms.log")
    pub fn from_env(env_file: Option<&str>) -> Result<Self, KwtSmsError> {
        let env_path = env_file.unwrap_or(".env");
        let file_vars = load_env_file(env_path);

        let get_var = |key: &str, default: Option<&str>| -> Option<String> {
            // Priority: env var > .env file > default
            if let Ok(val) = std::env::var(key) {
                if !val.is_empty() {
                    return Some(val);
                }
            }
            if let Some(val) = file_vars.get(key) {
                return Some(val.clone());
            }
            default.map(|d| d.to_string())
        };

        let username = get_var("KWTSMS_USERNAME", None).ok_or_else(|| {
            KwtSmsError::InvalidInput(
                "KWTSMS_USERNAME not found in environment or .env file".to_string(),
            )
        })?;

        let password = get_var("KWTSMS_PASSWORD", None).ok_or_else(|| {
            KwtSmsError::InvalidInput(
                "KWTSMS_PASSWORD not found in environment or .env file".to_string(),
            )
        })?;

        let sender_id = get_var("KWTSMS_SENDER_ID", Some("KWT-SMS")).unwrap();
        let test_mode_str = get_var("KWTSMS_TEST_MODE", Some("0")).unwrap();
        let test_mode = test_mode_str == "1" || test_mode_str.to_lowercase() == "true";
        let log_file = get_var("KWTSMS_LOG_FILE", Some("kwtsms.log")).unwrap();

        KwtSms::new(
            &username,
            &password,
            Some(&sender_id),
            test_mode,
            Some(&log_file),
        )
    }

    /// Get the cached balance from the last verify() or successful send().
    pub fn cached_balance(&self) -> Option<f64> {
        self.cached_balance.lock().ok().and_then(|g| *g)
    }

    /// Get the cached purchased credits from the last verify().
    pub fn cached_purchased(&self) -> Option<f64> {
        self.cached_purchased.lock().ok().and_then(|g| *g)
    }

    /// Test credentials and get balance.
    ///
    /// Returns `VerifyResult` with `ok`, `balance`, `purchased`, `error`.
    /// Never panics.
    pub fn verify(&self) -> VerifyResult {
        let payload = json!({
            "username": self.username,
            "password": self.password,
        });

        match api_request("balance", &payload, &self.log_file) {
            Ok(data) => {
                let result = data
                    .get("result")
                    .and_then(|v| v.as_str())
                    .unwrap_or("ERROR");

                if result == "OK" {
                    let available = data
                        .get("available")
                        .and_then(|v| v.as_f64());
                    let purchased = data
                        .get("purchased")
                        .and_then(|v| v.as_f64());

                    if let Ok(mut bal) = self.cached_balance.lock() {
                        *bal = available;
                    }
                    if let Ok(mut pur) = self.cached_purchased.lock() {
                        *pur = purchased;
                    }

                    VerifyResult {
                        ok: true,
                        balance: available,
                        purchased,
                        error: None,
                    }
                } else {
                    let desc = data
                        .get("description")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown error");
                    let action = data
                        .get("action")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    let error_msg = if action.is_empty() {
                        desc.to_string()
                    } else {
                        format!("{} {}", desc, action)
                    };

                    VerifyResult {
                        ok: false,
                        balance: None,
                        purchased: None,
                        error: Some(error_msg),
                    }
                }
            }
            Err(e) => VerifyResult {
                ok: false,
                balance: None,
                purchased: None,
                error: Some(e.to_string()),
            },
        }
    }

    /// Get current balance. Returns `None` on error (falls back to cached value).
    pub fn balance(&self) -> Option<f64> {
        let result = self.verify();
        if result.ok {
            result.balance
        } else {
            self.cached_balance()
        }
    }

    /// Send SMS to one or more phone numbers.
    ///
    /// - Accepts a single number string or a slice of numbers.
    /// - Automatically normalizes and validates all phone numbers.
    /// - Cleans the message (strips emojis, HTML, invisible chars, converts Arabic digits).
    /// - If >200 valid numbers, auto-routes to bulk send with batching.
    /// - Deduplicates normalized numbers.
    ///
    /// # Arguments
    /// - `mobile`: Phone number(s). Single string or comma-separated, or a slice of strings.
    /// - `message`: SMS text.
    /// - `sender`: Override sender ID (None uses default).
    pub fn send(
        &self,
        mobile: &[&str],
        message: &str,
        sender: Option<&str>,
    ) -> Result<Value, KwtSmsError> {
        let cleaned = clean_message(message);
        if cleaned.trim().is_empty() {
            return Ok(json!({
                "result": "ERROR",
                "code": "ERR009",
                "description": "Message is empty after cleaning.",
                "action": API_ERRORS.get("ERR009").unwrap_or(&"")
            }));
        }

        let sender = sender.unwrap_or(&self.sender_id);
        let mut valid_numbers: Vec<String> = Vec::new();
        let mut invalid_entries: Vec<InvalidEntry> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();

        // Flatten: each entry may be comma-separated
        for &entry in mobile {
            for part in entry.split(',') {
                let part = part.trim();
                if part.is_empty() {
                    continue;
                }
                let (is_valid, error, normalized) =
                    validate_phone_input(&part.to_string());

                if is_valid {
                    if seen.insert(normalized.clone()) {
                        valid_numbers.push(normalized);
                    }
                } else {
                    invalid_entries.push(InvalidEntry {
                        input: part.to_string(),
                        error: error.unwrap_or_default(),
                    });
                }
            }
        }

        if valid_numbers.is_empty() {
            return Ok(json!({
                "result": "ERROR",
                "code": "ERR_INVALID_INPUT",
                "description": "No valid phone numbers provided.",
                "action": API_ERRORS.get("ERR_INVALID_INPUT").unwrap_or(&""),
                "invalid": invalid_entries.iter().map(|e| json!({"input": e.input, "error": e.error})).collect::<Vec<_>>()
            }));
        }

        if valid_numbers.len() > MAX_BATCH_SIZE {
            return self.send_bulk(&valid_numbers, &cleaned, sender, &invalid_entries);
        }

        let mobile_str = valid_numbers.join(",");

        let payload = json!({
            "username": self.username,
            "password": self.password,
            "sender": sender,
            "mobile": mobile_str,
            "message": cleaned,
            "test": if self.test_mode { "1" } else { "0" },
        });

        match api_request("send", &payload, &self.log_file) {
            Ok(mut data) => {
                // Update cached balance from response
                if let Some(bal) = data.get("balance-after").and_then(|v| v.as_f64()) {
                    if let Ok(mut cached) = self.cached_balance.lock() {
                        *cached = Some(bal);
                    }
                }

                if !invalid_entries.is_empty() {
                    if let Some(obj) = data.as_object_mut() {
                        obj.insert(
                            "invalid".to_string(),
                            serde_json::to_value(&invalid_entries).unwrap_or(Value::Array(vec![])),
                        );
                    }
                }

                Ok(data)
            }
            Err(KwtSmsError::Api {
                code,
                description,
                action,
            }) => Ok(json!({
                "result": "ERROR",
                "code": code,
                "description": description,
                "action": action,
                "invalid": invalid_entries.iter().map(|e| json!({"input": e.input, "error": e.error})).collect::<Vec<_>>()
            })),
            Err(e) => Ok(json!({
                "result": "ERROR",
                "description": e.to_string(),
                "invalid": invalid_entries.iter().map(|e| json!({"input": e.input, "error": e.error})).collect::<Vec<_>>()
            })),
        }
    }

    /// Convenience: send to a single number string (may be comma-separated).
    pub fn send_one(
        &self,
        mobile: &str,
        message: &str,
        sender: Option<&str>,
    ) -> Result<Value, KwtSmsError> {
        self.send(&[mobile], message, sender)
    }

    /// Bulk send: auto-batches >200 numbers with delay.
    fn send_bulk(
        &self,
        numbers: &[String],
        message: &str,
        sender: &str,
        invalid_entries: &[InvalidEntry],
    ) -> Result<Value, KwtSmsError> {
        let batches: Vec<&[String]> = numbers.chunks(MAX_BATCH_SIZE).collect();
        let total_batches = batches.len() as u32;

        let mut msg_ids: Vec<String> = Vec::new();
        let mut errors: Vec<Value> = Vec::new();
        let mut total_numbers: u32 = 0;
        let mut total_points: u32 = 0;
        let mut last_balance: Option<f64> = None;
        let mut successes: u32 = 0;

        for (i, batch) in batches.iter().enumerate() {
            if i > 0 {
                thread::sleep(Duration::from_millis(BATCH_DELAY_MS));
            }

            let mobile_str = batch.join(",");
            let payload = json!({
                "username": self.username,
                "password": self.password,
                "sender": sender,
                "mobile": mobile_str,
                "message": message,
                "test": if self.test_mode { "1" } else { "0" },
            });

            let batch_result = self.send_batch_with_retry(&payload, i as u32 + 1);

            match batch_result {
                Ok(data) => {
                    let result = data
                        .get("result")
                        .and_then(|v| v.as_str())
                        .unwrap_or("ERROR");

                    if result == "OK" {
                        successes += 1;
                        if let Some(id) = data.get("msg-id").and_then(|v| v.as_str()) {
                            msg_ids.push(id.to_string());
                        }
                        if let Some(n) = data.get("numbers").and_then(|v| v.as_u64()) {
                            total_numbers += n as u32;
                        }
                        if let Some(p) = data.get("points-charged").and_then(|v| v.as_u64()) {
                            total_points += p as u32;
                        }
                        if let Some(b) = data.get("balance-after").and_then(|v| v.as_f64()) {
                            last_balance = Some(b);
                        }
                    } else {
                        let code = data
                            .get("code")
                            .and_then(|v| v.as_str())
                            .unwrap_or("UNKNOWN");
                        let desc = data
                            .get("description")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Unknown error");
                        errors.push(json!({
                            "batch": i + 1,
                            "code": code,
                            "description": desc
                        }));
                    }
                }
                Err(e) => {
                    errors.push(json!({
                        "batch": i + 1,
                        "code": "NETWORK",
                        "description": e.to_string()
                    }));
                }
            }
        }

        if let Some(bal) = last_balance {
            if let Ok(mut cached) = self.cached_balance.lock() {
                *cached = Some(bal);
            }
        }

        let result_status = if successes == total_batches {
            "OK"
        } else if successes == 0 {
            "ERROR"
        } else {
            "PARTIAL"
        };

        let mut response = json!({
            "result": result_status,
            "bulk": true,
            "batches": total_batches,
            "numbers": total_numbers,
            "points-charged": total_points,
            "balance-after": last_balance,
            "msg-ids": msg_ids,
            "errors": errors,
        });

        if !invalid_entries.is_empty() {
            if let Some(obj) = response.as_object_mut() {
                obj.insert(
                    "invalid".to_string(),
                    serde_json::to_value(invalid_entries).unwrap_or(Value::Array(vec![])),
                );
            }
        }

        Ok(response)
    }

    /// Send a single batch with ERR013 retry logic.
    fn send_batch_with_retry(
        &self,
        payload: &Value,
        _batch_num: u32,
    ) -> Result<Value, KwtSmsError> {
        let mut last_result = api_request("send", payload, &self.log_file);

        for backoff in ERR013_BACKOFF_SECS {
            match &last_result {
                Ok(data) => {
                    let code = data.get("code").and_then(|v| v.as_str()).unwrap_or("");
                    if code != "ERR013" {
                        return last_result;
                    }
                    // ERR013: queue full, retry with backoff
                    thread::sleep(Duration::from_secs(*backoff));
                    last_result = api_request("send", payload, &self.log_file);
                }
                Err(_) => return last_result,
            }
        }

        last_result
    }

    /// Send with auto-retry on ERR028 (15-second rate limit).
    ///
    /// Retries up to `max_retries` times with 16-second delay.
    /// Only retries ERR028. All other errors return immediately.
    pub fn send_with_retry(
        &self,
        mobile: &[&str],
        message: &str,
        sender: Option<&str>,
        max_retries: u32,
    ) -> Result<Value, KwtSmsError> {
        let mut result = self.send(mobile, message, sender)?;

        for _ in 0..max_retries {
            let code = result
                .get("code")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if code != "ERR028" {
                return Ok(result);
            }
            thread::sleep(Duration::from_secs(16));
            result = self.send(mobile, message, sender)?;
        }

        Ok(result)
    }

    /// Validate phone numbers via the API.
    ///
    /// Pre-validates locally first. Invalid numbers are collected in `rejected`
    /// and never sent to the API.
    pub fn validate(&self, phones: &[&str]) -> Result<Value, KwtSmsError> {
        let mut valid_numbers: Vec<String> = Vec::new();
        let mut rejected: Vec<InvalidEntry> = Vec::new();
        let mut seen: HashSet<String> = HashSet::new();

        for &phone in phones {
            for part in phone.split(',') {
                let part = part.trim();
                if part.is_empty() {
                    continue;
                }
                let (is_valid, error, normalized) = validate_phone_input(part);
                if is_valid {
                    if seen.insert(normalized.clone()) {
                        valid_numbers.push(normalized);
                    }
                } else {
                    rejected.push(InvalidEntry {
                        input: part.to_string(),
                        error: error.unwrap_or_default(),
                    });
                }
            }
        }

        if valid_numbers.is_empty() {
            return Ok(json!({
                "ok": [],
                "er": [],
                "nr": [],
                "raw": null,
                "error": "No valid phone numbers to validate.",
                "rejected": rejected.iter().map(|e| json!({"input": e.input, "error": e.error})).collect::<Vec<_>>()
            }));
        }

        let mobile_str = valid_numbers.join(",");
        let payload = json!({
            "username": self.username,
            "password": self.password,
            "mobile": mobile_str,
        });

        match api_request("validate", &payload, &self.log_file) {
            Ok(data) => {
                let result = data
                    .get("result")
                    .and_then(|v| v.as_str())
                    .unwrap_or("ERROR");

                if result == "OK" {
                    let mobile_data = data.get("mobile").cloned().unwrap_or(json!({}));
                    let ok_list = mobile_data
                        .get("OK")
                        .and_then(|v| v.as_array())
                        .map(|a| {
                            a.iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    let er_list = mobile_data
                        .get("ER")
                        .and_then(|v| v.as_array())
                        .map(|a| {
                            a.iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();
                    let nr_list = mobile_data
                        .get("NR")
                        .and_then(|v| v.as_array())
                        .map(|a| {
                            a.iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default();

                    Ok(json!({
                        "ok": ok_list,
                        "er": er_list,
                        "nr": nr_list,
                        "raw": data,
                        "error": null,
                        "rejected": rejected.iter().map(|e| json!({"input": e.input, "error": e.error})).collect::<Vec<_>>()
                    }))
                } else {
                    let desc = data
                        .get("description")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown error");
                    let action = data.get("action").and_then(|v| v.as_str()).unwrap_or("");
                    let error_msg = if action.is_empty() {
                        desc.to_string()
                    } else {
                        format!("{} {}", desc, action)
                    };

                    Ok(json!({
                        "ok": [],
                        "er": [],
                        "nr": [],
                        "raw": data,
                        "error": error_msg,
                        "rejected": rejected.iter().map(|e| json!({"input": e.input, "error": e.error})).collect::<Vec<_>>()
                    }))
                }
            }
            Err(e) => Ok(json!({
                "ok": [],
                "er": [],
                "nr": [],
                "raw": null,
                "error": e.to_string(),
                "rejected": rejected.iter().map(|e| json!({"input": e.input, "error": e.error})).collect::<Vec<_>>()
            })),
        }
    }

    /// List registered sender IDs.
    pub fn senderids(&self) -> Result<Value, KwtSmsError> {
        let payload = json!({
            "username": self.username,
            "password": self.password,
        });

        match api_request("senderid", &payload, &self.log_file) {
            Ok(mut data) => {
                // Normalize response structure
                let result = data
                    .get("result")
                    .and_then(|v| v.as_str())
                    .unwrap_or("ERROR")
                    .to_string();

                if result == "OK" {
                    // API returns "senderid" (singular), normalize to "senderids" (plural)
                    let senderids = data
                        .get("senderid")
                        .cloned()
                        .unwrap_or(json!([]));
                    if let Some(obj) = data.as_object_mut() {
                        obj.insert("senderids".to_string(), senderids);
                    }
                }

                Ok(data)
            }
            Err(KwtSmsError::Api {
                code,
                description,
                action,
            }) => Ok(json!({
                "result": "ERROR",
                "code": code,
                "description": description,
                "action": action
            })),
            Err(e) => Ok(json!({
                "result": "ERROR",
                "description": e.to_string()
            })),
        }
    }

    /// List active country prefixes.
    pub fn coverage(&self) -> Result<Value, KwtSmsError> {
        let payload = json!({
            "username": self.username,
            "password": self.password,
        });

        match api_request("coverage", &payload, &self.log_file) {
            Ok(data) => Ok(data),
            Err(KwtSmsError::Api {
                code,
                description,
                action,
            }) => Ok(json!({
                "result": "ERROR",
                "code": code,
                "description": description,
                "action": action
            })),
            Err(e) => Ok(json!({
                "result": "ERROR",
                "description": e.to_string()
            })),
        }
    }

    /// Get message queue status.
    pub fn status(&self, msg_id: &str) -> Result<Value, KwtSmsError> {
        if msg_id.trim().is_empty() {
            return Ok(json!({
                "result": "ERROR",
                "code": "ERR_INVALID_INPUT",
                "description": "Message ID is required."
            }));
        }

        let payload = json!({
            "username": self.username,
            "password": self.password,
            "msgid": msg_id,
        });

        match api_request("status", &payload, &self.log_file) {
            Ok(data) => Ok(data),
            Err(KwtSmsError::Api {
                code,
                description,
                action,
            }) => Ok(json!({
                "result": "ERROR",
                "code": code,
                "description": description,
                "action": action
            })),
            Err(e) => Ok(json!({
                "result": "ERROR",
                "description": e.to_string()
            })),
        }
    }

    /// Get delivery report (international numbers only).
    pub fn dlr(&self, msg_id: &str) -> Result<Value, KwtSmsError> {
        if msg_id.trim().is_empty() {
            return Ok(json!({
                "result": "ERROR",
                "code": "ERR_INVALID_INPUT",
                "description": "Message ID is required."
            }));
        }

        let payload = json!({
            "username": self.username,
            "password": self.password,
            "msgid": msg_id,
        });

        match api_request("dlr", &payload, &self.log_file) {
            Ok(data) => Ok(data),
            Err(KwtSmsError::Api {
                code,
                description,
                action,
            }) => Ok(json!({
                "result": "ERROR",
                "code": code,
                "description": description,
                "action": action
            })),
            Err(e) => Ok(json!({
                "result": "ERROR",
                "description": e.to_string()
            })),
        }
    }
}

// KwtSms is Send + Sync because it uses Arc<Mutex<>> for interior mutability
unsafe impl Send for KwtSms {}
unsafe impl Sync for KwtSms {}
