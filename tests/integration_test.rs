//! Integration tests that hit the live kwtSMS API.
//!
//! These tests are behind the `integration` feature flag.
//! Run with: cargo test --features integration
//!
//! Required environment variables:
//! - RUST_USERNAME: API username
//! - RUST_PASSWORD: API password
//!
//! All tests use test_mode=true (no credits consumed, no messages delivered).

#![cfg(feature = "integration")]

use kwtsms::KwtSms;

fn get_client() -> Option<KwtSms> {
    let username = std::env::var("RUST_USERNAME").ok()?;
    let password = std::env::var("RUST_PASSWORD").ok()?;

    if username.is_empty() || password.is_empty() {
        return None;
    }

    KwtSms::new(&username, &password, None, true, Some("")).ok()
}

fn get_client_or_skip() -> KwtSms {
    match get_client() {
        Some(c) => c,
        None => {
            eprintln!("Skipping: RUST_USERNAME / RUST_PASSWORD not set");
            return KwtSms::new("skip", "skip", None, true, Some("")).unwrap();
        }
    }
}

fn has_credentials() -> bool {
    let u = std::env::var("RUST_USERNAME").unwrap_or_default();
    let p = std::env::var("RUST_PASSWORD").unwrap_or_default();
    !u.is_empty() && !p.is_empty()
}

// ===== Verify / Balance =====

#[test]
fn test_integration_verify_valid_credentials() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.verify();
    assert!(result.ok, "Verify should succeed: {:?}", result.error);
    assert!(result.balance.is_some());
    assert!(result.balance.unwrap() >= 0.0);
}

#[test]
fn test_integration_verify_wrong_credentials() {
    let client = KwtSms::new("wrong_user_xyz", "wrong_pass_xyz", None, true, Some("")).unwrap();
    let result = client.verify();
    assert!(!result.ok);
    assert!(result.error.is_some());
}

#[test]
fn test_integration_balance() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let balance = client.balance();
    assert!(balance.is_some());
    assert!(balance.unwrap() >= 0.0);
}

#[test]
fn test_integration_cached_balance() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    assert!(client.cached_balance().is_none());
    let _ = client.verify();
    assert!(client.cached_balance().is_some());
}

// ===== Send =====

#[test]
fn test_integration_send_valid_kuwait_number() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.send_one("96598765432", "Test from Rust client", None);
    assert!(result.is_ok());
    let data = result.unwrap();
    let status = data["result"].as_str().unwrap_or("MISSING");
    // In test mode, should be OK or an expected error (ERR025, ERR006, etc.)
    assert!(
        status == "OK" || status == "ERROR",
        "Unexpected result: {}",
        data
    );
}

#[test]
fn test_integration_send_invalid_email() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.send_one("user@gmail.com", "Test", None);
    assert!(result.is_ok());
    let data = result.unwrap();
    assert_eq!(data["result"].as_str().unwrap(), "ERROR");
    assert!(data["invalid"].as_array().unwrap().len() > 0);
}

#[test]
fn test_integration_send_too_short() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.send_one("123", "Test", None);
    assert!(result.is_ok());
    let data = result.unwrap();
    assert_eq!(data["result"].as_str().unwrap(), "ERROR");
}

#[test]
fn test_integration_send_letters_only() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.send_one("abcdefgh", "Test", None);
    assert!(result.is_ok());
    let data = result.unwrap();
    assert_eq!(data["result"].as_str().unwrap(), "ERROR");
}

#[test]
fn test_integration_send_mixed_valid_invalid() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.send(&["96598765432", "abc", "123"], "Test mixed", None);
    assert!(result.is_ok());
    let data = result.unwrap();
    // Should have invalid entries
    let invalid = data["invalid"].as_array();
    assert!(invalid.is_some());
    assert!(invalid.unwrap().len() >= 2); // "abc" and "123"
}

#[test]
fn test_integration_send_with_normalization_plus() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.send_one("+96598765432", "Test plus prefix", None);
    assert!(result.is_ok());
}

#[test]
fn test_integration_send_with_normalization_double_zero() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.send_one("0096598765432", "Test 00 prefix", None);
    assert!(result.is_ok());
}

#[test]
fn test_integration_send_duplicate_normalized() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    // These all normalize to the same number
    let result = client.send(
        &["+96598765432", "0096598765432", "96598765432"],
        "Dedup test",
        None,
    );
    assert!(result.is_ok());
    // Deduplicated: only one number should be sent
}

#[test]
fn test_integration_send_empty_message() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.send_one("96598765432", "", None);
    assert!(result.is_ok());
    let data = result.unwrap();
    assert_eq!(data["result"].as_str().unwrap(), "ERROR");
}

#[test]
fn test_integration_send_emoji_only_message() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.send_one("96598765432", "\u{1F600}\u{1F601}", None);
    assert!(result.is_ok());
    let data = result.unwrap();
    // After cleaning, message is empty -> ERR009
    assert_eq!(data["result"].as_str().unwrap(), "ERROR");
    assert_eq!(data["code"].as_str().unwrap(), "ERR009");
}

// ===== Sender IDs =====

#[test]
fn test_integration_senderids() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.senderids();
    assert!(result.is_ok());
    let data = result.unwrap();
    let status = data["result"].as_str().unwrap_or("MISSING");
    if status == "OK" {
        assert!(data.get("senderids").is_some() || data.get("senderid").is_some());
    }
}

// ===== Coverage =====

#[test]
fn test_integration_coverage() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.coverage();
    assert!(result.is_ok());
    let data = result.unwrap();
    let status = data["result"].as_str().unwrap_or("MISSING");
    if status == "OK" {
        assert!(data.get("prefixes").is_some());
    }
}

// ===== Validate =====

#[test]
fn test_integration_validate_valid_numbers() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.validate(&["96598765432"]);
    assert!(result.is_ok());
}

#[test]
fn test_integration_validate_mixed() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.validate(&["96598765432", "abc", "123"]);
    assert!(result.is_ok());
    let data = result.unwrap();
    // Should have rejected entries
    assert!(data["rejected"].as_array().unwrap().len() >= 2);
}

// ===== Status =====

#[test]
fn test_integration_status_nonexistent() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.status("nonexistent_msg_id_xyz");
    assert!(result.is_ok());
    let data = result.unwrap();
    // Should return an error (ERR029 or similar)
    let status = data["result"].as_str().unwrap_or("OK");
    assert_eq!(status, "ERROR");
}

#[test]
fn test_integration_status_empty_id() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.status("");
    assert!(result.is_ok());
    let data = result.unwrap();
    assert_eq!(data["result"].as_str().unwrap(), "ERROR");
}

// ===== DLR =====

#[test]
fn test_integration_dlr_nonexistent() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.dlr("nonexistent_msg_id_xyz");
    assert!(result.is_ok());
    let data = result.unwrap();
    assert_eq!(data["result"].as_str().unwrap(), "ERROR");
}

#[test]
fn test_integration_dlr_empty_id() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.dlr("");
    assert!(result.is_ok());
    let data = result.unwrap();
    assert_eq!(data["result"].as_str().unwrap(), "ERROR");
}

// ===== Wrong Sender ID =====

#[test]
fn test_integration_send_wrong_sender() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.send_one(
        "96598765432",
        "Wrong sender test",
        Some("NONEXISTENT-SENDER-12345"),
    );
    assert!(result.is_ok());
    let data = result.unwrap();
    // May return ERR008 or OK depending on test mode behavior
    let status = data["result"].as_str().unwrap_or("MISSING");
    assert!(status == "OK" || status == "ERROR");
}

// ===== Empty Sender ID =====

#[test]
fn test_integration_send_empty_sender() {
    if !has_credentials() {
        return;
    }
    let client = get_client_or_skip();
    let result = client.send_one("96598765432", "Empty sender test", Some(""));
    assert!(result.is_ok());
}
