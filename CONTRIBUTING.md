# Contributing to kwtsms-rust

## Development Setup

1. Install Rust: https://rustup.rs
2. Clone the repository:
   ```
   git clone https://github.com/boxlinknet/kwtsms-rust.git
   cd kwtsms-rust
   ```
3. Build: `cargo build`
4. Run tests: `cargo test`

## Running Tests

### Unit and mock tests (no credentials needed)

```
cargo test
```

### Integration tests (real API, test mode)

```
export RUST_USERNAME=your_api_user
export RUST_PASSWORD=your_api_pass
cargo test --features integration
```

Integration tests always use `test_mode=true`. No credits are consumed.

### CLI

```
cargo run --features cli -- verify
cargo run --features cli -- send 96598765432 "Hello"
```

## Code Style

- Run `cargo fmt` before committing.
- Run `cargo clippy` and fix all warnings.
- All public functions need doc comments.

## Pull Request Checklist

- [ ] `cargo build` succeeds
- [ ] `cargo test` passes
- [ ] `cargo clippy` has no warnings
- [ ] `cargo fmt --check` passes
- [ ] New features have tests
- [ ] CHANGELOG.md updated
