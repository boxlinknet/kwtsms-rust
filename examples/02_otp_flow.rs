use kwtsms::{validate_phone_input, KwtSms};

fn main() {
    let sms = KwtSms::from_env(None).expect("Failed to create client");

    // 1. Validate the phone number locally first
    let phone = "+96598765432";
    let (valid, error, normalized) = validate_phone_input(phone);
    if !valid {
        eprintln!("Invalid phone: {}", error.unwrap());
        return;
    }
    println!("Normalized phone: {}", normalized);

    // 2. Generate OTP (use your own secure random generator)
    let otp = "123456";

    // 3. Send OTP with your app name (telecom compliance requirement)
    let message = format!("Your OTP for MyApp is: {}. Valid for 5 minutes.", otp);
    let result = sms
        .send_one(&normalized, &message, None)
        .expect("Send failed");

    if result["result"].as_str() == Some("OK") {
        println!("OTP sent successfully!");
        if let Some(msg_id) = result["msg-id"].as_str() {
            println!("Message ID: {} (save this for status checks)", msg_id);
        }
    } else {
        eprintln!("Failed to send OTP: {}", result);
        if let Some(action) = result["action"].as_str() {
            eprintln!("Action: {}", action);
        }
    }

    // 4. Important reminders:
    // - Use a Transactional Sender ID for OTP (not Promotional, not KWT-SMS)
    // - Implement rate limiting: max 3-5 OTP per phone per hour
    // - OTP expiry: 3-5 minutes
    // - On resend: generate a NEW code, invalidate the old one
    // - Minimum resend timer: 3-4 minutes (KNET standard is 4 minutes)
}
