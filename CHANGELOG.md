# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.1] - 2026-03-06

### Changed

- Add Dependabot configuration for automated dependency updates.
- CI/CD auto-publish pipeline test.

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
