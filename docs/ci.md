# CI

Rustipo uses GitHub Actions for continuous integration.

Workflow file:

- `.github/workflows/ci.yml`

Checks run on push and pull request:

- `cargo fmt -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`
