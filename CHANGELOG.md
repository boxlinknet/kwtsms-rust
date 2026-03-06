# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.7] - 2026-03-06

### Added

- `05_otp_production` example: production OTP flow with rate limiting, expiry, brute-force protection, user-friendly error mapping, and database schemas (PostgreSQL, MySQL, SQLite, Redis).

## [0.1.6] - 2026-03-06

### Changed

- Replace CodeQL workflow with `cargo audit` security scan.
- Add Security Audit badge to README.
- Update CONTRIBUTING project structure to reflect cargo audit workflow.

## [0.1.5] - 2026-03-06

### Added

- SECURITY.md with vulnerability reporting instructions.
- `cargo audit` security workflow (weekly + push/PR).
- examples/README.md index file.
- Full README per PRD: About kwtSMS, Prerequisites, Credential Management, Input Sanitization, Error Handling, Phone Number Formats, Test Mode, Sender ID, Best Practices, Security Checklist, What's Handled Automatically, FAQ, Help & Support.
- Full CONTRIBUTING.md per PRD: project structure, branch naming, commit style, PR checklist, security issues.

### Changed

- Use `rust_username`/`rust_password` env vars for integration tests (per-language convention).
- Replace CodeQL with `cargo audit` (CodeQL does not support Rust).
- Simplify `.gitignore`: collapse docs rules into `docs/`.

## [0.1.4] - 2026-03-06

### Added

- Badges in README (crates.io, docs.rs, CI, MSRV, license).

## [0.1.3] - 2026-03-06

### Fixed

- Bump MSRV to 1.83 (required by ureq transitive dependencies: icu_collections, idna).

## [0.1.2] - 2026-03-06

### Fixed

- Fix cargo fmt and clippy warnings for CI compliance.
- Update MSRV to 1.80 (required for LazyLock).

## [0.1.1] - 2026-03-06

### Changed

- Add Dependabot configuration for automated dependency updates.

## [0.1.0] - 2026-03-06

### Added

- Initial release.
- `KwtSms` client with all 7 API endpoints: send, balance/verify, validate, senderid, coverage, status, dlr.
- Automatic phone number normalization and validation.
- Message cleaning: strips emojis, HTML, invisible characters, converts Arabic digits.
- Bulk send with auto-batching (>200 numbers), 0.5s delay, ERR013 retry with backoff.
- All 33 error codes mapped to developer-friendly action messages.
- `.env` file support with `from_env()` factory method.
- JSONL logging with password masking.
- Thread-safe cached balance.
- CLI binary behind `cli` feature flag.
- Comprehensive test suite: unit, mock, and integration tests.
