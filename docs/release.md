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
- When a release is created, the workflow syncs the GitHub release body from the generated Release Please notes.
- The same workflow builds prebuilt binary archives for the main supported targets and uploads them, plus a SHA-256 checksum file, to the GitHub release.
- The repository also contains the source-of-truth Homebrew formula at `Formula/rustipo.rb`.
- When `HOMEBREW_TAP_TOKEN` is configured in GitHub Actions secrets, the release workflow syncs that formula into `fcendesu/homebrew-rustipo` after the release assets are uploaded.
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
10. Confirm the GitHub release page includes the generated notes body.
11. Confirm the GitHub release assets include the platform archives and the SHA-256 checksum file.
12. Sync local `master` again.

```bash
git checkout master
git pull --ff-only origin master
```

13. Validate the publishable crate from the merged release state.

```bash
cargo package --list
cargo publish --dry-run
```

14. Publish to crates.io.

```bash
cargo publish
```

15. Verify the published crate version on crates.io.
16. Verify the corresponding GitHub release and changelog entry.

## Homebrew formula maintenance

Rustipo publishes Homebrew installs through the separate tap repository `fcendesu/homebrew-rustipo`.
macOS users can install it with:

```bash
brew install fcendesu/rustipo/rustipo
```

The main repository keeps the source-of-truth formula in `Formula/rustipo.rb`.

### Required secret

To let the release workflow update the public tap automatically, configure this repository secret:

- `HOMEBREW_TAP_TOKEN`
  - use a dedicated fine-grained GitHub personal access token
  - scope it only to `fcendesu/homebrew-rustipo`
  - grant:
    - repository contents: `Read and write`
    - metadata: `Read`
  - do not reuse a broad personal token from your main developer account if you can avoid it

### Automatic sync behavior

After a release is finalized and the GitHub release assets exist, the release workflow will:

1. run `./scripts/update-homebrew-formula.sh <tag>` in the Rustipo checkout
2. copy the resulting `Formula/rustipo.rb` into `fcendesu/homebrew-rustipo`
3. commit and push the tap update automatically

### Manual fallback

If the secret is missing or the automated sync fails, you can still update the tap manually:

1. Update the formula from the release checksum file.

```bash
./scripts/update-homebrew-formula.sh rustipo-v0.11.0
```

The script downloads the public release checksum file directly from GitHub Releases and does not
require `gh auth` or a maintainer token.

2. Review the resulting `Formula/rustipo.rb` change.
3. Copy that file into `fcendesu/homebrew-rustipo/Formula/rustipo.rb`.
4. Commit and push the tap repo update.
5. Validate the public tap.

```bash
brew tap fcendesu/rustipo
brew install fcendesu/rustipo/rustipo
brew test rustipo
brew uninstall --force rustipo
brew untap fcendesu/rustipo
```

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
- automate Homebrew formula updates after release artifacts are available
