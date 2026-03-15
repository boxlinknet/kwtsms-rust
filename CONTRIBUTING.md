# Contributing to kwtsms-rust

Contributions are welcome: bug reports, fixes, new examples, and documentation improvements.

## Before You Start

- Search existing issues before opening a new one
- Open an issue before starting large changes
- All contributions must pass the test suite

## Development Setup

### Prerequisites

- Rust 1.83+ with Cargo: https://rustup.rs

### Clone and verify

```bash
git clone https://github.com/boxlinknet/kwtsms-rust.git
cd kwtsms-rust
cargo build
cargo test
cargo clippy
```

## Running Tests

### Tier 1: Unit tests (no credentials needed)

```bash
cargo test
```

Tests pure functions: `normalize_phone()`, `validate_phone_input()`, `clean_message()`, error mapping.

### Tier 2: Mocked API tests (no credentials needed)

Included in `cargo test`. Tests in `tests/errors_test.rs` verify error code mapping and enrichment.

### Tier 3: Integration tests (real API, test mode)

```bash
export rust_username=rust_username
export rust_password=rust_password
cargo test --features integration
```

Integration tests always use `test_mode=true`. No credits are consumed.

## Build

```bash
cargo build              # debug build
cargo build --release    # release build
```

Output: `target/debug/` or `target/release/`

## Project Structure

```
kwtsms-rust/
├── Cargo.toml                    Package metadata, dependencies, features
├── README.md                     Full documentation
├── CHANGELOG.md                  Version history (Keep a Changelog format)
├── CONTRIBUTING.md               This file
├── SECURITY.md                   Security vulnerability reporting
├── LICENSE                       MIT license
├── .gitignore                    Excluded files
├── .env.example                  Example environment variables
├── .github/
│   ├── dependabot.yml            Automated dependency updates
│   └── workflows/
│       ├── publish.yml           CI: test + cargo publish on tag
│       └── codeql.yml            Cargo audit security scan (weekly + push/PR)
├── src/
│   ├── lib.rs                    Public re-exports
│   ├── client.rs                 KwtSms struct, all API methods
│   ├── phone.rs                  normalize_phone(), validate_phone_input()
│   ├── message.rs                clean_message()
│   ├── errors.rs                 API_ERRORS map, enrich_error(), KwtSmsError
│   ├── request.rs                HTTP POST via ureq, reads 4xx bodies
│   ├── env.rs                    load_env_file()
│   ├── logger.rs                 JSONL logger with password masking
│   ├── types.rs                  All public return types
├── tests/
│   ├── phone_test.rs             Phone normalization + validation (66 tests)
│   ├── message_test.rs           Message cleaning (45 tests)
│   ├── errors_test.rs            Error code mapping (13 tests)
│   └── integration_test.rs       Real API tests (behind feature flag)
└── examples/
    ├── README.md                 Example index
    ├── 01_basic_usage.rs         Verify, send, balance
    ├── 02_otp_flow.rs            OTP send with validation
    ├── 03_bulk_sms.rs            Bulk send with batching
    ├── 04_error_handling.rs      Error paths, user-facing messages
    └── 05_otp_production.rs      Production OTP: rate limiting, expiry, DB schemas
```

## Making Changes

### Branch naming

```
fix/short-description        bug fix
feat/short-description       new feature
docs/short-description       documentation only
test/short-description       tests only
chore/short-description      build, tooling, dependency updates
```

### Commit style (Conventional Commits)

```
<type>: <short description>

[optional body]
```

Types: `feat`, `fix`, `docs`, `test`, `refactor`, `chore`

Examples:
```
feat: add status() method for message queue lookup
fix: handle ERR028 (15s same-number cooldown) in bulk send
test: cover Arabic digit normalization edge cases
chore: bump dependency versions
```

### Code style

- Run `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- All public functions need doc comments
- Zero new dependencies unless absolutely necessary

## Adding a New Method

1. Write a failing test in `tests/` for the new behavior
2. Run `cargo test` and verify it fails
3. Implement the method in `src/client.rs`
4. Run `cargo test` and verify it passes
5. Export the return type from `src/types.rs` and `src/lib.rs`
6. Add documentation to README.md
7. Update CHANGELOG.md under `[Unreleased]`

## Pull Request Process

1. Fork the repository
2. Create a feature branch from `main`
3. Make your changes
4. Run the full test suite
5. Open a PR with a clear description

### PR checklist

```
- [ ] Tests added/updated for all changed behavior
- [ ] All existing tests pass (`cargo test`)
- [ ] Build succeeds (`cargo build`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Code formatted (`cargo fmt --check`)
- [ ] CHANGELOG.md updated under [Unreleased]
- [ ] No new runtime dependencies added
- [ ] Public types exported if new public types added
```

## Reporting Bugs

Include:
- Rust version (`rustc --version`)
- kwtsms version (from `Cargo.toml`)
- Minimal code to reproduce the issue
- Expected vs actual behavior
- Full error output

## Security Issues

Do not open public issues for security vulnerabilities. Use [GitHub's private security advisory](https://github.com/boxlinknet/kwtsms-rust/security/advisories/new) or contact support directly.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
