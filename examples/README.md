# Examples

Runnable examples for the `kwtsms` Rust client.

## Running Examples

```bash
# Set up credentials first
export KWTSMS_USERNAME=rust_username
export KWTSMS_PASSWORD=rust_password
export KWTSMS_TEST_MODE=1

# Run an example
cargo run --example basic_usage
cargo run --example otp_flow
cargo run --example bulk_sms
cargo run --example error_handling
```

## Examples

| # | Example | File | Credentials needed? |
|---|---------|------|---------------------|
| 01 | [Basic Usage](01_basic_usage.rs) | Verify credentials, send SMS, check balance | Yes |
| 02 | [OTP Flow](02_otp_flow.rs) | Validate phone number, send OTP with best practices | Yes |
| 03 | [Bulk SMS](03_bulk_sms.rs) | Bulk send with >200 number auto-batching | Yes |
| 04 | [Error Handling](04_error_handling.rs) | All error paths, user-facing error mapping | Yes |
| 05 | [OTP Production](05_otp_production.rs) | Production OTP: rate limiting, expiry, DB schemas (PostgreSQL, MySQL, SQLite, Redis) | Yes |

All examples use `KWTSMS_TEST_MODE=1` by default (no real messages sent, no credits consumed).
