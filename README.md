# kwtsms

[![Crates.io](https://img.shields.io/crates/v/kwtsms.svg)](https://crates.io/crates/kwtsms)
[![docs.rs](https://docs.rs/kwtsms/badge.svg)](https://docs.rs/kwtsms)
[![CI](https://github.com/boxlinknet/kwtsms-rust/actions/workflows/publish.yml/badge.svg)](https://github.com/boxlinknet/kwtsms-rust/actions)
[![MSRV](https://img.shields.io/badge/MSRV-1.83-blue.svg)](https://blog.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Rust client for the [kwtSMS API](https://www.kwtsms.com). Send SMS, check balance, validate numbers, list sender IDs, check coverage, get delivery reports.

## Install

```toml
# Cargo.toml
[dependencies]
kwtsms = "0.1"
```

Or: `cargo add kwtsms`

**Minimum supported Rust version: 1.83**

## Quick Start

```rust
use kwtsms::KwtSms;

fn main() {
    // Load credentials from .env file or environment variables
    let sms = KwtSms::from_env(None).unwrap();

    // Verify credentials
    let result = sms.verify();
    println!("Balance: {:?}", result.balance);

    // Send SMS
    let response = sms.send_one("96598765432", "Hello from Rust!", None).unwrap();
    println!("{}", response);
}
```

## Environment Variables

Create a `.env` file or set these environment variables:

```ini
KWTSMS_USERNAME=your_api_user
KWTSMS_PASSWORD=your_api_pass
KWTSMS_SENDER_ID=KWT-SMS
KWTSMS_TEST_MODE=1
KWTSMS_LOG_FILE=kwtsms.log
```

Or pass credentials directly:

```rust
let sms = KwtSms::new("username", "password", Some("MY-SENDER"), false, None).unwrap();
```

## API Reference

### Verify Credentials

```rust
let result = sms.verify();
if result.ok {
    println!("Balance: {}", result.balance.unwrap());
    println!("Purchased: {}", result.purchased.unwrap());
}
```

### Send SMS

```rust
// Single number
let result = sms.send_one("96598765432", "Hello!", None).unwrap();

// Multiple numbers
let result = sms.send(&["96598765432", "96512345678"], "Hello!", None).unwrap();

// Custom sender ID
let result = sms.send_one("96598765432", "Hello!", Some("MY-APP")).unwrap();

// Comma-separated
let result = sms.send_one("96598765432,96512345678", "Hello!", None).unwrap();
```

The send method automatically:
- Normalizes phone numbers (strips +, 00, spaces, dashes, converts Arabic digits)
- Validates each number locally (rejects emails, too short/long, no digits)
- Cleans the message (strips emojis, HTML, invisible chars, converts Arabic digits)
- Deduplicates normalized numbers
- Auto-batches when >200 numbers (200 per batch, 0.5s delay)

### Check Balance

```rust
let balance = sms.balance(); // Option<f64>
```

### Validate Numbers

```rust
let result = sms.validate(&["96598765432", "96512345678"]).unwrap();
// Returns: ok[], er[], nr[], rejected[]
```

### Sender IDs

```rust
let result = sms.senderids().unwrap();
```

### Coverage

```rust
let result = sms.coverage().unwrap();
```

### Message Status

```rust
let result = sms.status("msg-id-from-send-response").unwrap();
```

### Delivery Report (international only)

```rust
let result = sms.dlr("msg-id-from-send-response").unwrap();
```

### Utility Functions

```rust
use kwtsms::{normalize_phone, validate_phone_input, clean_message};

// Normalize phone number
let phone = normalize_phone("+965 9876-5432"); // "96598765432"

// Validate phone input
let (valid, error, normalized) = validate_phone_input("user@gmail.com");
// valid=false, error="'user@gmail.com' is an email address..."

// Clean message text
let cleaned = clean_message("Hello \u{1F600} OTP: \u{0661}\u{0662}\u{0663}");
// "Hello  OTP: 123"
```

### Error Codes

All 33 kwtSMS error codes are mapped to developer-friendly action messages:

```rust
use kwtsms::{API_ERRORS, enrich_error};

// Access the error map directly
if let Some(action) = API_ERRORS.get("ERR003") {
    println!("{}", action);
    // "Wrong API username or password. Check KWTSMS_USERNAME and KWTSMS_PASSWORD..."
}
```

## Credential Management

**Never hardcode credentials.** Use one of these approaches:

1. **Environment variables / .env file** (default): `KwtSms::from_env(None)` loads from env vars, then `.env` file.
2. **Constructor injection**: `KwtSms::new(username, password, ...)` for custom config systems.
3. **Secrets manager**: Load from AWS Secrets Manager, Vault, etc., then pass to the constructor.

## Best Practices

### Always save msg-id and balance-after

```rust
let result = sms.send_one("96598765432", "Hello!", None).unwrap();
if result["result"] == "OK" {
    // Save msg-id for status checks and delivery reports
    let msg_id = result["msg-id"].as_str().unwrap();
    // Save balance-after: never call balance() after send()
    let balance = result["balance-after"].as_f64().unwrap();
}
```

### Use Transactional Sender ID for OTP

`KWT-SMS` is a shared test sender. Register a private Transactional sender ID for OTP messages. Promotional sender IDs are blocked by DND on Zain and Ooredoo, causing silent delivery failure.

### Validate locally before calling the API

```rust
use kwtsms::validate_phone_input;

let (valid, error, normalized) = validate_phone_input(user_input);
if !valid {
    // Return error to user without hitting the API
}
```

### Security Checklist

Before going live:

- [ ] Bot protection enabled (CAPTCHA for web)
- [ ] Rate limit per phone number (max 3-5/hour)
- [ ] Rate limit per IP address (max 10-20/hour)
- [ ] Rate limit per user/session if authenticated
- [ ] Monitoring/alerting on abuse patterns
- [ ] Admin notification on low balance
- [ ] Test mode OFF (`KWTSMS_TEST_MODE=0`)
- [ ] Private Sender ID registered (not KWT-SMS)
- [ ] Transactional Sender ID for OTP (not promotional)

## CLI

Build the CLI binary:

```
cargo install kwtsms --features cli
```

Commands:

```
kwtsms verify                                          # test credentials
kwtsms balance                                         # show credits
kwtsms senderid                                        # list sender IDs
kwtsms coverage                                        # list active prefixes
kwtsms send <mobile> <message> [--sender ID]           # send SMS
kwtsms validate <number> [number ...]                  # validate numbers
kwtsms status <msg-id>                                 # check status
kwtsms dlr <msg-id>                                    # delivery report
```

## Testing

```bash
# Unit + mock tests (no credentials needed)
cargo test

# Integration tests (real API, test mode, no credits consumed)
export rust_username=your_api_user
export rust_password=your_api_pass
cargo test --features integration
```

## License

MIT
