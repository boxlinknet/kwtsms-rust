use kwtsms::KwtSms;

fn main() {
    // Create client from environment variables / .env file
    let sms = KwtSms::from_env(None).expect("Failed to create client");

    // Verify credentials and check balance
    let verify = sms.verify();
    if verify.ok {
        println!("Credentials OK!");
        println!("Balance: {:?}", verify.balance);
        println!("Purchased: {:?}", verify.purchased);
    } else {
        eprintln!("Verification failed: {:?}", verify.error);
        return;
    }

    // Send a single SMS
    let result = sms
        .send_one("96598765432", "Hello from the kwtsms Rust client!", None)
        .expect("Send failed");

    println!(
        "Send result: {}",
        serde_json::to_string_pretty(&result).unwrap()
    );

    // Check the msg-id and balance-after from the response
    if let Some(msg_id) = result.get("msg-id").and_then(|v| v.as_str()) {
        println!("Message ID: {}", msg_id);
        // Save this msg-id for status checks and delivery reports
    }
    if let Some(balance) = result.get("balance-after").and_then(|v| v.as_f64()) {
        println!("Balance after send: {}", balance);
        // Save to your database: no need to call balance() again
    }
}
