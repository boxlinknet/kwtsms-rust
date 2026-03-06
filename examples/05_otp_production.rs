//! Production OTP flow with rate limiting, expiry, brute-force protection,
//! user-friendly error messages, and database storage patterns.
//!
//! This example uses in-memory HashMap as the OTP store. In production,
//! replace it with your database of choice. See the "Database Choices"
//! section below for SQL schemas and recommended crates.
//!
//! Run: cargo run --example otp_production

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use kwtsms::{validate_phone_input, KwtSms};

// ============================================================================
// Configuration
// ============================================================================

const APP_NAME: &str = "MyApp";
const OTP_LENGTH: usize = 6;
const OTP_EXPIRY: Duration = Duration::from_secs(300); // 5 minutes
const RESEND_COOLDOWN: Duration = Duration::from_secs(240); // 4 minutes (KNET standard)
const MAX_ATTEMPTS_PER_PHONE: u32 = 3; // per hour
const MAX_ATTEMPTS_PER_IP: u32 = 10; // per hour
const LOW_BALANCE_THRESHOLD: f64 = 50.0;
const SENDER_ID: &str = "MyApp"; // YOUR registered Transactional Sender ID

// ============================================================================
// Database Choices
// ============================================================================
//
// Replace the in-memory OtpStore with one of these in production.
// All schemas include automatic expiry and rate limiting.
//
// ---- Option 1: PostgreSQL (recommended for most apps) ----
// Crate: sqlx = { version = "0.8", features = ["runtime-tokio", "postgres"] }
//
// CREATE TABLE otp_codes (
//     phone       VARCHAR(20) PRIMARY KEY,
//     code        VARCHAR(10) NOT NULL,
//     attempts    INT DEFAULT 0,
//     created_at  TIMESTAMPTZ DEFAULT NOW(),
//     expires_at  TIMESTAMPTZ DEFAULT NOW() + INTERVAL '5 minutes'
// );
// CREATE INDEX idx_otp_expires ON otp_codes (expires_at);
//
// -- Cleanup expired codes (run via pg_cron or app-side):
// -- DELETE FROM otp_codes WHERE expires_at < NOW();
//
// CREATE TABLE sms_rate_limits (
//     key         VARCHAR(64) PRIMARY KEY,  -- "phone:96598765432" or "ip:1.2.3.4"
//     count       INT DEFAULT 1,
//     window_start TIMESTAMPTZ DEFAULT NOW()
// );
//
// CREATE TABLE sms_log (
//     id          SERIAL PRIMARY KEY,
//     msg_id      VARCHAR(64) NOT NULL,
//     phone       VARCHAR(20) NOT NULL,
//     purpose     VARCHAR(20) DEFAULT 'otp',
//     balance     DOUBLE PRECISION,
//     created_at  TIMESTAMPTZ DEFAULT NOW()
// );
//
// ---- Option 2: MySQL / MariaDB ----
// Crate: sqlx = { version = "0.8", features = ["runtime-tokio", "mysql"] }
//
// CREATE TABLE otp_codes (
//     phone       VARCHAR(20) PRIMARY KEY,
//     code        VARCHAR(10) NOT NULL,
//     attempts    INT DEFAULT 0,
//     created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
//     expires_at  TIMESTAMP DEFAULT (CURRENT_TIMESTAMP + INTERVAL 5 MINUTE)
// );
//
// ---- Option 3: SQLite (single-server, small apps) ----
// Crate: rusqlite = "0.32"
//
// CREATE TABLE otp_codes (
//     phone       TEXT PRIMARY KEY,
//     code        TEXT NOT NULL,
//     attempts    INTEGER DEFAULT 0,
//     created_at  TEXT DEFAULT (datetime('now')),
//     expires_at  TEXT DEFAULT (datetime('now', '+5 minutes'))
// );
//
// ---- Option 4: Redis (fastest, auto-expiry built in) ----
// Crate: redis = "0.27"
//
// SET   otp:96598765432  "123456"  EX 300        -- auto-expires in 5 min
// INCR  otp_attempts:96598765432                  -- brute-force counter
// EXPIRE otp_attempts:96598765432 300
// INCR  rate:phone:96598765432                    -- rate limit counter
// EXPIRE rate:phone:96598765432 3600              -- 1-hour window
// INCR  rate:ip:192.168.1.100
// EXPIRE rate:ip:192.168.1.100 3600
//
// ============================================================================

// ============================================================================
// OTP Store (in-memory — replace with DB above in production)
// ============================================================================

struct OtpRecord {
    code: String,
    created_at: Instant,
    attempts: u32,
}

struct RateLimit {
    count: u32,
    window_start: Instant,
}

struct OtpStore {
    codes: HashMap<String, OtpRecord>,
    phone_rates: HashMap<String, RateLimit>,
    ip_rates: HashMap<String, RateLimit>,
}

impl OtpStore {
    fn new() -> Self {
        Self {
            codes: HashMap::new(),
            phone_rates: HashMap::new(),
            ip_rates: HashMap::new(),
        }
    }
}

// ============================================================================
// Rate Limiting
// ============================================================================

fn check_rate_limit(
    rates: &mut HashMap<String, RateLimit>,
    key: &str,
    max: u32,
) -> Result<(), String> {
    let now = Instant::now();
    let hour = Duration::from_secs(3600);

    let entry = rates.entry(key.to_string()).or_insert(RateLimit {
        count: 0,
        window_start: now,
    });

    // Reset window if expired
    if now.duration_since(entry.window_start) > hour {
        entry.count = 0;
        entry.window_start = now;
    }

    if entry.count >= max {
        return Err("Too many requests. Please try again later.".to_string());
    }

    entry.count += 1;
    Ok(())
}

// ============================================================================
// OTP Generation
// ============================================================================

fn generate_otp() -> String {
    // PRODUCTION: use a cryptographically secure random generator:
    //
    //   # Cargo.toml: rand = "0.8"
    //   use rand::Rng;
    //   let code: u32 = rand::thread_rng().gen_range(100_000..1_000_000);
    //   format!("{:0>6}", code)
    //
    // DEMO: simple approach (NOT cryptographically secure):
    let seed = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    format!(
        "{:0>width$}",
        seed % 10u32.pow(OTP_LENGTH as u32),
        width = OTP_LENGTH
    )
}

// ============================================================================
// Send OTP
// ============================================================================

fn send_otp(
    sms: &KwtSms,
    store: &Mutex<OtpStore>,
    phone: &str,
    client_ip: &str,
) -> Result<String, String> {
    // 1. Validate phone locally (no API call wasted)
    let (valid, error, normalized) = validate_phone_input(phone);
    if !valid {
        return Err(format!(
            "Please enter a valid phone number (e.g., +965 9876 5432). {}",
            error.unwrap_or_default()
        ));
    }

    let mut store = store.lock().unwrap();

    // 2. Rate limit by IP
    check_rate_limit(&mut store.ip_rates, client_ip, MAX_ATTEMPTS_PER_IP)?;

    // 3. Rate limit by phone
    check_rate_limit(&mut store.phone_rates, &normalized, MAX_ATTEMPTS_PER_PHONE)?;

    // 4. Check resend cooldown (KNET standard: 4 minutes)
    if let Some(existing) = store.codes.get(&normalized) {
        let elapsed = Instant::now().duration_since(existing.created_at);
        if elapsed < RESEND_COOLDOWN {
            let remaining = RESEND_COOLDOWN - elapsed;
            return Err(format!(
                "Please wait {} seconds before requesting another code.",
                remaining.as_secs()
            ));
        }
    }

    // 5. Generate new OTP (always fresh — invalidate previous)
    let otp = generate_otp();

    // 6. Send via kwtSMS with Transactional Sender ID
    //    - Always include app name (telecom compliance)
    //    - One number per OTP request (never batch OTPs)
    let message = format!(
        "Your {} verification code is: {}. Valid for {} minutes.",
        APP_NAME,
        otp,
        OTP_EXPIRY.as_secs() / 60
    );

    let result = sms
        .send_one(&normalized, &message, Some(SENDER_ID))
        .map_err(|e| {
            format!(
                "SMS service is temporarily unavailable. Please try again later. ({})",
                e
            )
        })?;

    match result["result"].as_str() {
        Some("OK") => {
            // 7. Save msg-id and balance-after (CRITICAL for production)
            //    - msg-id: needed to check delivery status later
            //    - balance-after: track balance without extra API calls
            if let Some(msg_id) = result["msg-id"].as_str() {
                println!("[LOG] OTP sent to {}. msg-id: {}", normalized, msg_id);
                // DB: INSERT INTO sms_log (msg_id, phone, purpose, balance)
                //     VALUES ($1, $2, 'otp', $3);
            }
            if let Some(balance) = result["balance-after"].as_f64() {
                println!("[LOG] Balance: {}", balance);
                // DB: UPDATE app_settings SET sms_balance = $1;
                if balance < LOW_BALANCE_THRESHOLD {
                    println!("[ALERT] Low SMS balance: {}!", balance);
                    // notify_admin("SMS balance below threshold", balance);
                }
            }

            // 8. Store OTP
            // DB: INSERT INTO otp_codes (phone, code) VALUES ($1, $2)
            //     ON CONFLICT (phone) DO UPDATE SET code=$2, attempts=0, created_at=NOW();
            // Redis: SET otp:$phone $code EX 300
            store.codes.insert(
                normalized.clone(),
                OtpRecord {
                    code: otp,
                    created_at: Instant::now(),
                    attempts: 0,
                },
            );

            Ok("Verification code sent.".to_string())
        }
        _ => {
            // 9. Map API errors to user-friendly messages
            //    NEVER show raw API errors to end users
            let code = result["code"].as_str().unwrap_or("");
            let user_msg = match code {
                "ERR006" | "ERR025" => "Please enter a valid phone number (e.g., +965 9876 5432).",
                "ERR010" | "ERR011" => {
                    println!("[ALERT] SMS balance depleted! Code: {}", code);
                    "SMS service is temporarily unavailable. Please try again later."
                }
                "ERR026" => "SMS delivery to this country is not available.",
                "ERR028" => "Please wait a moment before requesting another code.",
                "ERR031" | "ERR032" => "Your message could not be sent. Please try again.",
                _ => {
                    println!("[ERROR] OTP send failed: {}", result);
                    "SMS service is temporarily unavailable. Please try again later."
                }
            };
            Err(user_msg.to_string())
        }
    }
}

// ============================================================================
// Verify OTP
// ============================================================================

fn verify_otp(store: &Mutex<OtpStore>, phone: &str, code: &str) -> Result<String, String> {
    let (_, _, normalized) = validate_phone_input(phone);

    let mut store = store.lock().unwrap();

    // DB: SELECT code, attempts, expires_at FROM otp_codes WHERE phone = $1;
    // Redis: GET otp:$phone
    let record = store
        .codes
        .get_mut(&normalized)
        .ok_or("No verification code found. Please request a new one.")?;

    // Check expiry
    // DB: WHERE expires_at > NOW()
    // Redis: automatic (TTL)
    if Instant::now().duration_since(record.created_at) > OTP_EXPIRY {
        store.codes.remove(&normalized);
        return Err("Code expired. Please request a new one.".to_string());
    }

    // Brute-force protection: max 5 verification attempts
    // DB: UPDATE otp_codes SET attempts = attempts + 1 WHERE phone = $1;
    // Redis: INCR otp_attempts:$phone
    record.attempts += 1;
    if record.attempts > 5 {
        store.codes.remove(&normalized);
        // DB: DELETE FROM otp_codes WHERE phone = $1;
        return Err("Too many incorrect attempts. Please request a new code.".to_string());
    }

    // Verify code
    if record.code != code {
        return Err(format!(
            "Invalid code. {} attempts remaining.",
            5 - record.attempts
        ));
    }

    // Success — delete used OTP immediately (one-time use)
    // DB: DELETE FROM otp_codes WHERE phone = $1;
    // Redis: DEL otp:$phone otp_attempts:$phone
    store.codes.remove(&normalized);
    Ok("Phone number verified.".to_string())
}

// ============================================================================
// Main — Simulates the full OTP lifecycle
// ============================================================================

fn main() {
    // In production: KwtSms::from_env(None) with KWTSMS_TEST_MODE=0
    let sms = KwtSms::new(
        "rust_username",
        "rust_password",
        Some(SENDER_ID),
        true, // test_mode: set to false in production
        None,
    )
    .expect("Failed to create client");

    let store = Mutex::new(OtpStore::new());
    let client_ip = "192.168.1.100"; // From your web framework (e.g., request.remote_addr)

    // Step 1: User requests OTP
    println!("=== Step 1: Request OTP ===");
    match send_otp(&sms, &store, "+965 9876 5432", client_ip) {
        Ok(msg) => println!("  {}", msg),
        Err(msg) => println!("  Error: {}", msg),
    }

    // Step 2: User enters wrong code
    println!("\n=== Step 2: Wrong code ===");
    match verify_otp(&store, "+965 9876 5432", "999999") {
        Ok(msg) => println!("  {}", msg),
        Err(msg) => println!("  Error: {}", msg),
    }

    // Step 3: User enters correct code
    println!("\n=== Step 3: Correct code ===");
    let correct_code = {
        let s = store.lock().unwrap();
        s.codes
            .get("96598765432")
            .map(|r| r.code.clone())
            .unwrap_or_else(|| "000000".to_string())
    };
    match verify_otp(&store, "+965 9876 5432", &correct_code) {
        Ok(msg) => println!("  {}", msg),
        Err(msg) => println!("  Error: {}", msg),
    }

    // Step 4: Rapid resend — blocked by cooldown
    println!("\n=== Step 4: Resend too soon ===");
    match send_otp(&sms, &store, "+965 9876 5432", client_ip) {
        Ok(msg) => println!("  {}", msg),
        Err(msg) => println!("  Error: {}", msg),
    }

    println!("\n=== Production Checklist ===");
    println!("  [ ] Transactional Sender ID registered (not KWT-SMS)");
    println!("  [ ] KWTSMS_TEST_MODE=0");
    println!("  [ ] CAPTCHA on OTP request form");
    println!(
        "  [ ] Rate limits: {} per phone/hour, {} per IP/hour",
        MAX_ATTEMPTS_PER_PHONE, MAX_ATTEMPTS_PER_IP
    );
    println!("  [ ] OTP expiry: {} minutes", OTP_EXPIRY.as_secs() / 60);
    println!(
        "  [ ] Resend cooldown: {} minutes",
        RESEND_COOLDOWN.as_secs() / 60
    );
    println!("  [ ] Database storage (not in-memory)");
    println!("  [ ] Low balance alerts configured");
    println!("  [ ] msg-id saved for every send");
}
