# kwtSMS Rust Client

[![Crates.io](https://img.shields.io/crates/v/kwtsms.svg)](https://crates.io/crates/kwtsms)
[![docs.rs](https://docs.rs/kwtsms/badge.svg)](https://docs.rs/kwtsms)
[![CI](https://github.com/boxlinknet/kwtsms-rust/actions/workflows/publish.yml/badge.svg)](https://github.com/boxlinknet/kwtsms-rust/actions)
[![Security Audit](https://github.com/boxlinknet/kwtsms-rust/actions/workflows/codeql.yml/badge.svg)](https://github.com/boxlinknet/kwtsms-rust/actions/workflows/codeql.yml)
[![MSRV](https://img.shields.io/badge/MSRV-1.83-blue.svg)](https://blog.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

Rust client for the [kwtSMS API](https://www.kwtsms.com). Send SMS, check balance, validate numbers, list sender IDs, check coverage, get delivery reports.

## About kwtSMS

kwtSMS is a Kuwaiti SMS gateway trusted by top businesses to deliver messages anywhere in the world, with private Sender ID, free API testing, non-expiring credits, and competitive flat-rate pricing. Secure, simple to integrate, built to last. Open a free account in under 1 minute, no paperwork or payment required. [Click here to get started](https://www.kwtsms.com/signup/)

## Prerequisites

You need **Rust** and **Cargo** (Rust's package manager) installed. They come together.

### Step 1: Check if Rust is installed

```bash
rustc --version
cargo --version
```

If you see version numbers, you're ready. If not, install Rust:

- **All platforms (recommended):**
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```
  Then restart your terminal.

- **Windows:** Download and run https://win.rustup.rs/

### Step 2: Create a project (if you don't have one)

```bash
cargo new my-project && cd my-project
```

### Step 3: Install kwtsms

```bash
cargo add kwtsms
```

Or add to your `Cargo.toml`:

```toml
[dependencies]
kwtsms = "0.1"
```

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

## Setup / Configuration

Create a `.env` file or set these environment variables:

```ini
KWTSMS_USERNAME=rust_username
KWTSMS_PASSWORD=rust_password
KWTSMS_SENDER_ID=KWT-SMS
KWTSMS_TEST_MODE=1
KWTSMS_LOG_FILE=kwtsms.log
```

Or pass credentials directly:

```rust
let sms = KwtSms::new("username", "password", Some("MY-SENDER"), false, None).unwrap();
```

## Credential Management

**Never hardcode credentials.** Use one of these approaches:

1. **Environment variables / .env file** (default): `KwtSms::from_env(None)` loads from env vars, then `.env` file. The file is `.gitignore`d and editable without redeployment.

2. **Constructor injection**: `KwtSms::new(username, password, ...)` for custom config systems, DI containers, or remote config.

3. **Secrets manager**: Load from AWS Secrets Manager, HashiCorp Vault, Google Secret Manager, or your own config API, then pass to the constructor.

4. **Admin settings UI** (for web apps): Store credentials in your database with a settings page. Include a "Test Connection" button that calls `verify()`.

## All Methods

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

### Check Balance

```rust
let balance = sms.balance(); // Option<f64>
let cached = sms.cached_balance(); // Option<f64>, from last verify/send
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

### Send with ERR028 Retry

```rust
// Auto-retries up to 3 times with 16s delay on ERR028 (same-number rate limit)
let result = sms.send_with_retry(&["96598765432"], "Hello!", None, 3).unwrap();
```

## Utility Functions

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

## Input Sanitization

`clean_message()` is called automatically by `send()` before every API call. It prevents the #1 cause of "message sent but not received" support tickets:

| Content | Effect without cleaning | What clean_message() does |
|---------|------------------------|--------------------------|
| Emojis | Stuck in queue, credits wasted, no error | Stripped |
| Hidden control characters (BOM, zero-width space, soft hyphen) | Spam filter rejection or queue stuck | Stripped |
| Arabic/Hindi numerals in body | OTP codes render inconsistently | Converted to Latin digits |
| HTML tags | ERR027, message rejected | Stripped |
| Directional marks (LTR, RTL) | May cause display issues | Stripped |

Arabic letters and Arabic text are fully supported and never stripped.

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

Test mode prints a visible warning before sending. Errors print `action` guidance.

## Error Handling

All methods return `Result<T, KwtSmsError>`. The error enum:

- `KwtSmsError::Network(String)`: connection/timeout errors
- `KwtSmsError::Api { code, description, action }`: API returned an error with developer guidance
- `KwtSmsError::InvalidInput(String)`: local validation failure

Every ERROR response includes an `action` field with a developer-friendly fix:

```rust
use kwtsms::{API_ERRORS, enrich_error};

if let Some(action) = API_ERRORS.get("ERR003") {
    println!("{}", action);
    // "Wrong API username or password. Check KWTSMS_USERNAME and KWTSMS_PASSWORD..."
}
```

### User-facing error mapping

Raw API errors should never be shown to end users. Map them:

| Situation | API error | Show to user |
|-----------|----------|--------------|
| Invalid phone number | ERR006, ERR025 | "Please enter a valid phone number in international format (e.g., +965 9876 5432)." |
| Wrong credentials | ERR003 | "SMS service is temporarily unavailable. Please try again later." (log + alert admin) |
| No balance | ERR010, ERR011 | "SMS service is temporarily unavailable. Please try again later." (alert admin) |
| Country not supported | ERR026 | "SMS delivery to this country is not available." |
| Rate limited | ERR028 | "Please wait a moment before requesting another code." |
| Message rejected | ERR031, ERR032 | "Your message could not be sent. Please try again with different content." |
| Queue full | ERR013 | "SMS service is busy. Please try again in a few minutes." (library retries automatically) |

## Phone Number Formats

All formats are accepted and normalized automatically:

| Input | Normalized | Valid? |
|-------|-----------|--------|
| `96598765432` | `96598765432` | Yes |
| `+96598765432` | `96598765432` | Yes |
| `0096598765432` | `96598765432` | Yes |
| `965 9876 5432` | `96598765432` | Yes |
| `965-9876-5432` | `96598765432` | Yes |
| `(965) 98765432` | `96598765432` | Yes |
| `٩٦٥٩٨٧٦٥٤٣٢` | `96598765432` | Yes |
| `123456` (too short) | rejected | No |
| `user@gmail.com` | rejected | No |

## Test Mode

**Test mode** (`KWTSMS_TEST_MODE=1`) sends your message to the kwtSMS queue but does NOT deliver it to the handset. No SMS credits are consumed. Use this during development.

**Live mode** (`KWTSMS_TEST_MODE=0`) delivers the message for real and deducts credits. Always develop in test mode and switch to live only when ready for production.

## Sender ID

A **Sender ID** is the name that appears as the sender on the recipient's phone (e.g., "MY-APP" instead of a random number).

| | Promotional | Transactional |
|--|-------------|---------------|
| **Use for** | Bulk SMS, marketing, offers | OTP, alerts, notifications |
| **Delivery to DND numbers** | Blocked/filtered, credits lost | Bypasses DND (whitelisted) |
| **Speed** | May have delays | Priority delivery |
| **Cost** | 10 KD one-time | 15 KD one-time |

`KWT-SMS` is a shared test sender. It causes delivery delays, is blocked on Virgin Kuwait, and should never be used in production. Register your own private Sender ID through your kwtSMS account. For OTP/authentication messages, you need a **Transactional** Sender ID to bypass DND filtering. Sender ID is **case sensitive**.

## Best Practices

### Always save msg-id and balance-after

```rust
let result = sms.send_one("96598765432", "Hello!", None).unwrap();
if result["result"] == "OK" {
    let msg_id = result["msg-id"].as_str().unwrap();     // save for status/DLR
    let balance = result["balance-after"].as_f64().unwrap(); // never call balance() after send()
}
```

### Validate locally before calling the API

```rust
use kwtsms::validate_phone_input;

let (valid, error, normalized) = validate_phone_input(user_input);
if !valid {
    // Return error to user without hitting the API
}
```

### Country coverage pre-check

Call `coverage()` once at startup and cache the active prefixes. Before every send, check if the number's country prefix is in the list. If not, return an error immediately without hitting the API.

### OTP requirements

- Always include app/company name: `"Your OTP for APPNAME is: 123456"`
- Resend timer: minimum 3-4 minutes (KNET standard is 4 minutes)
- OTP expiry: 3-5 minutes
- New code on resend: always generate a fresh code, invalidate previous
- Use Transactional Sender ID for OTP (not Promotional, not KWT-SMS)
- One number per OTP request: never batch OTP sends

## Security Checklist

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

## What's Handled Automatically

- **Phone normalization**: `+`, `00`, spaces, dashes, dots, parentheses stripped. Arabic-Indic digits converted. Leading zeros removed.
- **Duplicate phone removal**: If the same number appears multiple times (in different formats), it is sent only once.
- **Message cleaning**: Emojis removed (surrogate-pair safe). Hidden control characters (BOM, zero-width spaces, directional marks) removed. HTML tags stripped. Arabic-Indic digits in message body converted to Latin.
- **Batch splitting**: More than 200 numbers are automatically split into batches of 200 with 0.5s delay between batches.
- **ERR013 retry**: Queue-full errors are automatically retried up to 3 times with exponential backoff (30s / 60s / 120s).
- **Error enrichment**: Every API error response includes an `action` field with a developer-friendly fix hint.
- **Credential masking**: Passwords are always masked as `***` in log files. Never exposed.
- **Never panics**: All public methods return `Result` with structured errors. They never panic on API errors.

## Examples

See the [`examples/`](examples/) directory:

| Example | Description |
|---------|-------------|
| [01_basic_usage](examples/01_basic_usage.rs) | Verify credentials, send SMS, check balance |
| [02_otp_flow](examples/02_otp_flow.rs) | Validate phone, send OTP with best practices |
| [03_bulk_sms](examples/03_bulk_sms.rs) | Bulk send with >200 number batching |
| [04_error_handling](examples/04_error_handling.rs) | All error paths, user-facing message mapping |
| [05_otp_production](examples/05_otp_production.rs) | Production OTP: rate limiting, expiry, DB schemas |

Run an example: `cargo run --example basic_usage`

## Testing

```bash
# Unit + mock tests (no credentials needed)
cargo test

# Integration tests (real API, test mode, no credits consumed)
export rust_username=rust_username
export rust_password=rust_password
cargo test --features integration
```

## FAQ

**1. My message was sent successfully (result: OK) but the recipient didn't receive it. What happened?**

Check the **Sending Queue** at [kwtsms.com](https://www.kwtsms.com/login/). If your message is stuck there, it was accepted by the API but not dispatched. Common causes are emoji in the message, hidden characters from copy-pasting, or spam filter triggers. Delete it from the queue to recover your credits. Also verify that `test` mode is off (`KWTSMS_TEST_MODE=0`). Test messages are queued but never delivered.

**2. What is the difference between Test mode and Live mode?**

**Test mode** (`KWTSMS_TEST_MODE=1`) sends your message to the kwtSMS queue but does NOT deliver it to the handset. No SMS credits are consumed. Use this during development. **Live mode** (`KWTSMS_TEST_MODE=0`) delivers the message for real and deducts credits. Always develop in test mode and switch to live only when ready for production.

**3. What is a Sender ID and why should I not use "KWT-SMS" in production?**

A **Sender ID** is the name that appears as the sender on the recipient's phone (e.g., "MY-APP" instead of a random number). `KWT-SMS` is a shared test sender. It causes delivery delays, is blocked on Virgin Kuwait, and should never be used in production. Register your own private Sender ID through your kwtSMS account. For OTP/authentication messages, you need a **Transactional** Sender ID to bypass DND (Do Not Disturb) filtering.

**4. I'm getting ERR003 "Authentication error". What's wrong?**

You are using the wrong credentials. The API requires your **API username and API password**, NOT your account mobile number. Log in to [kwtsms.com](https://www.kwtsms.com/login/), go to Account, and check your API credentials. Also make sure you are using POST (not GET) and `Content-Type: application/json`.

**5. Can I send to international numbers (outside Kuwait)?**

International sending is **disabled by default** on kwtSMS accounts. Contact kwtSMS support to request activation for specific country prefixes. Use `coverage()` to check which countries are currently active on your account. Be aware that activating international coverage increases exposure to automated abuse. Implement rate limiting and CAPTCHA before enabling.

## Help & Support

- **[kwtSMS FAQ](https://www.kwtsms.com/faq/)**: Answers to common questions about credits, sender IDs, OTP, and delivery
- **[kwtSMS Support](https://www.kwtsms.com/support.html)**: Open a support ticket or browse help articles
- **[Contact kwtSMS](https://www.kwtsms.com/#contact)**: Reach the kwtSMS team directly for Sender ID registration and account issues
- **[API Documentation (PDF)](https://www.kwtsms.com/doc/KwtSMS.com_API_Documentation_v41.pdf)**: kwtSMS REST API v4.1 full reference
- **[kwtSMS Dashboard](https://www.kwtsms.com/login/)**: Recharge credits, buy Sender IDs, view message logs, manage coverage
- **[Other Integrations](https://www.kwtsms.com/integrations.html)**: Plugins and integrations for other platforms and languages

## License

MIT
