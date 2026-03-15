//! # kwtsms
//!
//! Rust client for the kwtSMS API (kwtsms.com).
//! Send SMS, check balance, validate numbers, list sender IDs, check coverage,
//! get delivery reports, and more.
//!
//! ## Quick Start
//!
//! ```no_run
//! use kwtsms::KwtSms;
//!
//! // Create client from .env file or environment variables
//! let sms = KwtSms::from_env(None).unwrap();
//!
//! // Verify credentials
//! let result = sms.verify();
//! println!("Balance: {:?}", result.balance);
//!
//! // Send SMS
//! let response = sms.send_one("96598765432", "Hello from Rust!", None).unwrap();
//! println!("{}", response);
//! ```

mod client;
pub(crate) mod env;
pub mod errors;
pub mod logger;
pub mod message;
pub mod phone;
mod request;
pub mod types;

// Re-export public API
pub use client::KwtSms;
pub use errors::{enrich_error, KwtSmsError, API_ERRORS};
pub use message::clean_message;
pub use phone::{
    find_country_code, normalize_phone, validate_phone_format, validate_phone_input, PhoneRule,
    PHONE_RULES,
};
pub use types::*;
