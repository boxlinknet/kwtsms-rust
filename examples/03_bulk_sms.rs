use kwtsms::KwtSms;

fn main() {
    let sms = KwtSms::from_env(None).expect("Failed to create client");

    // Bulk send: auto-batches when >200 numbers
    // Each batch is 200 numbers max, with 0.5s delay between batches
    let numbers: Vec<&str> = vec![
        "96598765432",
        "96512345678",
        "96587654321",
        // ... add more numbers
    ];

    let result = sms
        .send(
            &numbers,
            "Holiday sale! 50% off everything at MyStore.",
            None,
        )
        .expect("Bulk send failed");

    println!("{}", serde_json::to_string_pretty(&result).unwrap());

    // For >200 numbers, the result includes:
    // - "bulk": true
    // - "batches": number of batches sent
    // - "result": "OK" (all succeeded), "PARTIAL" (some failed), or "ERROR" (all failed)
    // - "msg-ids": array of message IDs from each batch
    // - "errors": array of per-batch errors if any
    // - "balance-after": balance after the last successful batch

    // Check for partial failures
    if let Some(errors) = result.get("errors").and_then(|v| v.as_array()) {
        if !errors.is_empty() {
            eprintln!("Some batches failed:");
            for err in errors {
                eprintln!(
                    "  Batch {}: {} - {}",
                    err["batch"],
                    err["code"].as_str().unwrap_or("?"),
                    err["description"].as_str().unwrap_or("?"),
                );
            }
        }
    }
}
