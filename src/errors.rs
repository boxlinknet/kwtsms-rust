use std::collections::HashMap;
use std::fmt;
use std::sync::LazyLock;

/// Custom error type for the kwtsms crate.
#[derive(Debug, Clone)]
pub enum KwtSmsError {
    /// Network or HTTP-level error.
    Network(String),
    /// API returned an error response with code, description, and action guidance.
    Api {
        code: String,
        description: String,
        action: String,
    },
    /// Invalid input provided by the caller.
    InvalidInput(String),
}

impl fmt::Display for KwtSmsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KwtSmsError::Network(msg) => write!(f, "Network error: {}", msg),
            KwtSmsError::Api {
                code,
                description,
                action,
            } => write!(f, "[{}] {}: {}", code, description, action),
            KwtSmsError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
        }
    }
}

impl std::error::Error for KwtSmsError {}

/// Complete error code map: kwtSMS error codes to developer-friendly action messages.
pub static API_ERRORS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert(
        "ERR001",
        "API is disabled on this account. Enable it at kwtsms.com \u{2192} Account \u{2192} API.",
    );
    m.insert("ERR002", "A required parameter is missing. Check that username, password, sender, mobile, and message are all provided.");
    m.insert("ERR003", "Wrong API username or password. Check KWTSMS_USERNAME and KWTSMS_PASSWORD. These are your API credentials, not your account mobile number.");
    m.insert(
        "ERR004",
        "This account does not have API access. Contact kwtSMS support to enable it.",
    );
    m.insert("ERR005", "This account is blocked. Contact kwtSMS support.");
    m.insert("ERR006", "No valid phone numbers. Make sure each number includes the country code (e.g., 96598765432 for Kuwait, not 98765432).");
    m.insert(
        "ERR007",
        "Too many numbers in a single request (maximum 200). Split into smaller batches.",
    );
    m.insert("ERR008", "This sender ID is banned or not found. Sender IDs are case sensitive. Check your registered sender IDs at kwtsms.com.");
    m.insert(
        "ERR009",
        "Message is empty. Provide a non-empty message text.",
    );
    m.insert(
        "ERR010",
        "Account balance is zero. Recharge credits at kwtsms.com.",
    );
    m.insert(
        "ERR011",
        "Insufficient balance for this send. Buy more credits at kwtsms.com.",
    );
    m.insert(
        "ERR012",
        "Message is too long (over 6 SMS pages). Shorten your message.",
    );
    m.insert(
        "ERR013",
        "Send queue is full (1000 messages). Wait a moment and try again.",
    );
    m.insert("ERR019", "No delivery reports found for this message.");
    m.insert(
        "ERR020",
        "Message ID does not exist. Make sure you saved the msg-id from the send response.",
    );
    m.insert(
        "ERR021",
        "No delivery report available for this message yet.",
    );
    m.insert(
        "ERR022",
        "Delivery reports are not ready yet. Try again after 24 hours.",
    );
    m.insert(
        "ERR023",
        "Unknown delivery report error. Contact kwtSMS support.",
    );
    m.insert("ERR024", "Your IP address is not in the API whitelist. Add it at kwtsms.com \u{2192} Account \u{2192} API \u{2192} IP Lockdown, or disable IP lockdown.");
    m.insert("ERR025", "Invalid phone number. Make sure the number includes the country code (e.g., 96598765432 for Kuwait, not 98765432).");
    m.insert("ERR026", "This country is not activated on your account. Contact kwtSMS support to enable the destination country.");
    m.insert(
        "ERR027",
        "HTML tags are not allowed in the message. Remove any HTML content and try again.",
    );
    m.insert("ERR028", "You must wait at least 15 seconds before sending to the same number again. No credits were consumed.");
    m.insert("ERR029", "Message ID does not exist or is incorrect.");
    m.insert("ERR030", "Message is stuck in the send queue with an error. Delete it at kwtsms.com \u{2192} Queue to recover credits.");
    m.insert("ERR031", "Message rejected: bad language detected.");
    m.insert("ERR032", "Message rejected: spam detected.");
    m.insert(
        "ERR033",
        "No active coverage found. Contact kwtSMS support.",
    );
    m.insert(
        "ERR_INVALID_INPUT",
        "One or more phone numbers are invalid. See details above.",
    );
    m
});

/// Enrich an API response with an `action` field if it contains an error code.
/// Returns a new JSON value with the action added. Does not mutate the original.
pub fn enrich_error(data: &mut serde_json::Value) {
    if let Some(obj) = data.as_object_mut() {
        let is_error = obj
            .get("result")
            .and_then(|v| v.as_str())
            .map(|s| s == "ERROR")
            .unwrap_or(false);

        if is_error {
            if let Some(code) = obj.get("code").and_then(|v| v.as_str()) {
                if let Some(action) = API_ERRORS.get(code) {
                    obj.insert(
                        "action".to_string(),
                        serde_json::Value::String(action.to_string()),
                    );
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_errors_has_all_codes() {
        assert!(API_ERRORS.contains_key("ERR001"));
        assert!(API_ERRORS.contains_key("ERR033"));
        assert!(API_ERRORS.contains_key("ERR_INVALID_INPUT"));
        assert_eq!(API_ERRORS.len(), 29);
    }

    #[test]
    fn test_enrich_error_adds_action() {
        let mut data = serde_json::json!({
            "result": "ERROR",
            "code": "ERR003",
            "description": "Authentication error"
        });
        enrich_error(&mut data);
        assert!(data["action"].as_str().unwrap().contains("KWTSMS_USERNAME"));
    }

    #[test]
    fn test_enrich_error_no_action_on_ok() {
        let mut data = serde_json::json!({
            "result": "OK",
            "balance": 100
        });
        enrich_error(&mut data);
        assert!(data.get("action").is_none());
    }

    #[test]
    fn test_enrich_error_unknown_code() {
        let mut data = serde_json::json!({
            "result": "ERROR",
            "code": "ERR999",
            "description": "Unknown error"
        });
        enrich_error(&mut data);
        assert!(data.get("action").is_none());
    }

    #[test]
    fn test_kwtsms_error_display() {
        let e = KwtSmsError::Network("timeout".to_string());
        assert!(e.to_string().contains("timeout"));

        let e = KwtSmsError::Api {
            code: "ERR003".to_string(),
            description: "Auth error".to_string(),
            action: "Check credentials".to_string(),
        };
        assert!(e.to_string().contains("ERR003"));

        let e = KwtSmsError::InvalidInput("bad phone".to_string());
        assert!(e.to_string().contains("bad phone"));
    }
}
