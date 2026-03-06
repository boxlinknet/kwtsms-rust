use kwtsms::{clean_message, validate_phone_input, KwtSms};

fn main() {
    let sms = KwtSms::from_env(None).expect("Failed to create client");

    // 1. Always validate phone numbers before sending
    let user_input = "+965 9876-5432";
    let (valid, error, normalized) = validate_phone_input(user_input);
    if !valid {
        // Show user-friendly message, not the raw error
        eprintln!(
            "Please enter a valid phone number in international format (e.g., +965 9876 5432)."
        );
        eprintln!("Debug: {}", error.unwrap());
        return;
    }
    println!("Normalized: {}", normalized);

    // 2. Clean message text before sending
    // clean_message() is called automatically by send(), but you can call it manually
    // to check the result or show a preview
    let raw_message = "Hello \u{1F600} Your code is \u{0661}\u{0662}\u{0663}\u{0664}";
    let cleaned = clean_message(raw_message);
    println!("Cleaned message: '{}'", cleaned);

    // 3. Send and handle errors
    let result = sms
        .send_one(&normalized, raw_message, None)
        .expect("Send failed");

    match result["result"].as_str() {
        Some("OK") => {
            println!("Sent! msg-id: {}", result["msg-id"]);
            println!("Balance: {}", result["balance-after"]);
        }
        Some("ERROR") => {
            let code = result["code"].as_str().unwrap_or("unknown");
            let desc = result["description"].as_str().unwrap_or("Unknown error");

            // Map API errors to user-facing messages
            match code {
                "ERR003" => {
                    // Auth error: don't tell the user about credentials
                    eprintln!("SMS service is temporarily unavailable. Please try again later.");
                    // Log the real error + alert admin
                }
                "ERR006" | "ERR025" => {
                    eprintln!("Please enter a valid phone number in international format.");
                }
                "ERR010" | "ERR011" => {
                    eprintln!("SMS service is temporarily unavailable. Please try again later.");
                    // Alert admin: balance is low/zero
                }
                "ERR026" => {
                    eprintln!("SMS delivery to this country is not available.");
                }
                "ERR028" => {
                    eprintln!("Please wait a moment before requesting another code.");
                }
                "ERR031" | "ERR032" => {
                    eprintln!(
                        "Your message could not be sent. Please try again with different content."
                    );
                }
                _ => {
                    eprintln!("Could not send SMS. Please try again later.");
                }
            }

            // Always log the real error for debugging
            eprintln!("Debug: [{}] {}", code, desc);
            if let Some(action) = result["action"].as_str() {
                eprintln!("Action: {}", action);
            }
        }
        _ => {
            eprintln!("Unexpected response: {}", result);
        }
    }

    // 4. Check for locally rejected numbers
    if let Some(invalid) = result.get("invalid").and_then(|v| v.as_array()) {
        for entry in invalid {
            eprintln!(
                "Rejected: {} - {}",
                entry["input"].as_str().unwrap_or("?"),
                entry["error"].as_str().unwrap_or("?")
            );
        }
    }
}
