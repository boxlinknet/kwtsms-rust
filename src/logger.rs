use serde_json;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::SystemTime;

/// Write a JSONL log entry. Never panics or crashes the main flow.
///
/// Entry fields: ts (UTC ISO-8601), endpoint, request (password masked), response, ok, error.
pub fn write_log(
    log_file: &str,
    endpoint: &str,
    request: &serde_json::Value,
    response: &serde_json::Value,
    ok: bool,
    error: Option<&str>,
) {
    if log_file.is_empty() {
        return;
    }

    let _ = write_log_inner(log_file, endpoint, request, response, ok, error);
}

fn write_log_inner(
    log_file: &str,
    endpoint: &str,
    request: &serde_json::Value,
    response: &serde_json::Value,
    ok: bool,
    error: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut masked_request = request.clone();
    if let Some(obj) = masked_request.as_object_mut() {
        if obj.contains_key("password") {
            obj.insert(
                "password".to_string(),
                serde_json::Value::String("***".to_string()),
            );
        }
    }

    let ts = format_utc_timestamp()?;

    let entry = serde_json::json!({
        "ts": ts,
        "endpoint": endpoint,
        "request": masked_request,
        "response": response,
        "ok": ok,
        "error": error,
    });

    let line = serde_json::to_string(&entry)?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?;

    writeln!(file, "{}", line)?;

    Ok(())
}

fn format_utc_timestamp() -> Result<String, Box<dyn std::error::Error>> {
    let now = SystemTime::now();
    let duration = now.duration_since(SystemTime::UNIX_EPOCH)?;
    let secs = duration.as_secs();
    let millis = duration.subsec_millis();

    // Calculate UTC date/time components from Unix timestamp
    let days = secs / 86400;
    let time_of_day = secs % 86400;
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;
    let seconds = time_of_day % 60;

    // Calculate year/month/day from days since epoch (1970-01-01)
    let (year, month, day) = days_to_date(days);

    Ok(format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:03}Z",
        year, month, day, hours, minutes, seconds, millis
    ))
}

fn days_to_date(days: u64) -> (u64, u64, u64) {
    // Algorithm from http://howardhinnant.github.io/date_algorithms.html
    let z = days + 719468;
    let era = z / 146097;
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_write_log_masks_password() {
        let path = format!("/tmp/kwtsms_test_log_{}.jsonl", std::process::id());
        let _ = fs::remove_file(&path);

        let request = serde_json::json!({
            "username": "testuser",
            "password": "secret123",
            "mobile": "96598765432"
        });
        let response = serde_json::json!({"result": "OK"});

        write_log(&path, "send", &request, &response, true, None);

        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("***"));
        assert!(!content.contains("secret123"));
        assert!(content.contains("testuser"));
        assert!(content.contains("send"));
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_write_log_empty_path_no_crash() {
        let request = serde_json::json!({});
        let response = serde_json::json!({});
        write_log("", "send", &request, &response, true, None);
        // Should not panic or crash
    }

    #[test]
    fn test_write_log_includes_timestamp() {
        let path = format!("/tmp/kwtsms_test_log_ts_{}.jsonl", std::process::id());
        let _ = fs::remove_file(&path);

        let request = serde_json::json!({"username": "test"});
        let response = serde_json::json!({"result": "OK"});

        write_log(&path, "balance", &request, &response, true, None);

        let content = fs::read_to_string(&path).unwrap();
        // Should contain UTC timestamp
        assert!(content.contains("T"));
        assert!(content.contains("Z"));
        fs::remove_file(&path).ok();
    }

    #[test]
    fn test_write_log_error_field() {
        let path = format!("/tmp/kwtsms_test_log_err_{}.jsonl", std::process::id());
        let _ = fs::remove_file(&path);

        let request = serde_json::json!({});
        let response = serde_json::json!({});

        write_log(
            &path,
            "send",
            &request,
            &response,
            false,
            Some("timeout"),
        );

        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("timeout"));
        assert!(content.contains("\"ok\":false"));
        fs::remove_file(&path).ok();
    }
}
