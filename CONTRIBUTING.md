# Contributing

Thanks for your interest in contributing to Rustipo.

## Development basics

- Use stable Rust.
- Keep changes focused and reviewable.
- Prefer small, single-intent commits.
- Add tests for changed behavior where practical.

## Workflow

1. Create a branch.
2. Make focused changes.
3. Run checks:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

4. Open a pull request with a clear description.

## Commit format

Use Conventional Commits:

`<type>(<scope>): <subject>`

Examples:

- `feat(cli): add build command`
- `fix(content): validate frontmatter`
- `chore(repo): update docs`

## PR expectations

- Explain what changed and why.
- Include test coverage notes.
- Keep refactors separate from behavior changes when practical.
