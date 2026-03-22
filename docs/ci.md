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
- When a release is created, the same workflow also builds prebuilt binaries for the main supported targets and uploads them, plus a checksum file, to the GitHub release.
- CI remains automatic for pushes and pull requests.

## Maintainer release docs

- [Release and publish workflow](./release.md)
