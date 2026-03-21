# Release And Publish Workflow

This document is for maintainers preparing a Rustipo release and publishing it to crates.io.

## Source of truth

- Release Please is the source of truth for release version bumps and changelog updates.
- Do not manually prepare a release by editing `Cargo.toml`, `CHANGELOG.md`, or `.github/release-please/manifest.json` first.
- Publish to crates.io from the merged release state on `master`, not from an older branch tip.

## Current release mechanics

- Release Please runs manually through GitHub Actions workflow dispatch.
- The workflow uses:
  - `.github/workflows/release-please.yml`
  - `.github/release-please/config.json`
  - `.github/release-please/manifest.json`
- The release PR updates:
  - `Cargo.toml`
  - `CHANGELOG.md`
  - `.github/release-please/manifest.json`

## Maintainer workflow

1. Make sure the intended release work is already merged to `master`.
2. Sync local `master`.

```bash
git checkout master
git pull --ff-only origin master
```

3. Run the normal repository checks.

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

4. Verify the crate package contents.

```bash
cargo package --list
```

5. Trigger the `Release Please` workflow manually in GitHub Actions.
6. Review the generated release PR.
7. Confirm the release PR contains the expected:
   - version bump
   - changelog section
   - manifest update
8. Merge the release PR.
9. Trigger the `Release Please` workflow manually again if needed to finalize the tag and GitHub release for the newly merged release commit.
10. Sync local `master` again.

```bash
git checkout master
git pull --ff-only origin master
```

11. Validate the publishable crate from the merged release state.

```bash
cargo package --list
cargo publish --dry-run
```

12. Publish to crates.io.

```bash
cargo publish
```

13. Verify the published crate version on crates.io.
14. Verify the corresponding GitHub release and changelog entry.

## Relationship between release prep and crates.io publish

- Release Please prepares and records the versioned release state.
- crates.io publish should happen after that release state is merged.
- The published crate version should match the merged release PR version.
- The GitHub release/tag and the crates.io release should refer to the same released version.

## Current validation status

Validated locally against the current repository state:

- `cargo package --allow-dirty --list`
- `cargo package --allow-dirty --no-verify`
- `cargo publish --dry-run --allow-dirty`

## Future automation ideas

- automate crates.io publish from a trusted release workflow after the release PR merge
- publish prebuilt binaries from the same release pipeline
- automate Homebrew formula updates after release artifacts are available
