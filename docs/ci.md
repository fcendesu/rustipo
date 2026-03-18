# CI

Rustipo uses GitHub Actions for continuous integration.

Workflow file:

- `.github/workflows/ci.yml`
- `.github/workflows/release-please.yml`

Checks run on push and pull request:

- `cargo fmt -- --check`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo test`

## Release cadence

- Release Please is scheduled weekly (Monday 09:00 UTC).
- Maintainers can run release automation manually via workflow dispatch when needed.
