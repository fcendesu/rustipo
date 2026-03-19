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

- Release Please runs manually via workflow dispatch when maintainers want to prepare a release.
- CI remains automatic for pushes and pull requests.
