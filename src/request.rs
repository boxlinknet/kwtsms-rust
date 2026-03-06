use crate::errors::{enrich_error, KwtSmsError};
use crate::logger::write_log;
use serde_json::Value;

const BASE_URL: &str = "https://www.kwtsms.com/API";
const TIMEOUT_SECS: u64 = 15;

/// POST to a kwtSMS API endpoint and return the parsed JSON response.
///
/// - Always POST, never GET
/// - Sets Content-Type and Accept to application/json
/// - Reads 4xx/5xx response bodies (kwtSMS returns JSON in 403 bodies)
/// - 15-second timeout
/// - Enriches ERROR responses with action guidance
/// - Logs every call to JSONL if log_file is not empty
pub fn api_request(
    endpoint: &str,
    payload: &Value,
    log_file: &str,
) -> Result<Value, KwtSmsError> {
    let url = format!("{}/{}/", BASE_URL, endpoint);

    let body = match serde_json::to_string(payload) {
        Ok(b) => b,
        Err(e) => return Err(KwtSmsError::Network(format!("JSON serialize error: {}", e))),
    };

    let result = ureq::post(&url)
        .set("Content-Type", "application/json")
        .set("Accept", "application/json")
        .timeout(std::time::Duration::from_secs(TIMEOUT_SECS))
        .send_string(&body);

    match result {
        Ok(response) => {
            let response_body = response
                .into_string()
                .map_err(|e| KwtSmsError::Network(format!("Failed to read response: {}", e)))?;

            let mut data: Value = serde_json::from_str(&response_body).map_err(|e| {
                KwtSmsError::Network(format!("Invalid JSON response: {}", e))
            })?;

            enrich_error(&mut data);

            let ok = data
                .get("result")
                .and_then(|v| v.as_str())
                .map(|s| s == "OK")
                .unwrap_or(false);

            write_log(log_file, endpoint, payload, &data, ok, None);

            Ok(data)
        }
        Err(ureq::Error::Status(status, response)) => {
            // Read 4xx/5xx response body: kwtSMS returns JSON error details
            let response_body = response
                .into_string()
                .unwrap_or_else(|_| format!("HTTP {} error", status));

            if let Ok(mut data) = serde_json::from_str::<Value>(&response_body) {
                enrich_error(&mut data);

                write_log(log_file, endpoint, payload, &data, false, None);

                // Return as API error if it has code and description
                if let (Some(code), Some(desc)) = (
                    data.get("code").and_then(|v| v.as_str()),
                    data.get("description").and_then(|v| v.as_str()),
                ) {
                    let action = data
                        .get("action")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    return Err(KwtSmsError::Api {
                        code: code.to_string(),
                        description: desc.to_string(),
                        action,
                    });
                }

                Ok(data)
            } else {
                let err_msg = format!("HTTP {}: {}", status, response_body);
                write_log(
                    log_file,
                    endpoint,
                    payload,
                    &Value::Null,
                    false,
                    Some(&err_msg),
                );
                Err(KwtSmsError::Network(err_msg))
            }
        }
        Err(ureq::Error::Transport(e)) => {
            let err_msg = format!("Network error: {}", e);
            write_log(
                log_file,
                endpoint,
                payload,
                &Value::Null,
                false,
                Some(&err_msg),
            );
            Err(KwtSmsError::Network(err_msg))
        }
    }
}
