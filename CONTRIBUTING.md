# Contributing

Thanks for your interest in contributing to Rustipo.

Rustipo is still a compact project, so clear, focused changes help a lot. Small improvements to
docs, examples, themes, tests, and core behavior are all valuable contributions.

Please also read the project's [Code of Conduct](./CODE_OF_CONDUCT.md).
If you need to report a security issue, use the private path described in [SECURITY.md](./SECURITY.md)
instead of filing a normal public issue first.

## Good starting points

- Read [README.md](./README.md) for the project overview and install story.
- Read [docs/roadmap.md](./docs/roadmap.md) to understand the current direction.
- Read [docs/project-structure.md](./docs/project-structure.md) and
  [docs/content-model.md](./docs/content-model.md) if your change affects generated sites.
- Read [docs/theme-contract.md](./docs/theme-contract.md) and
  [docs/theme-tera.md](./docs/theme-tera.md) if your change affects themes or template context.

## Development basics

- Use stable Rust.
- Keep changes focused and reviewable.
- Prefer small, single-intent commits.
- Add tests for changed behavior where practical.

## Local setup

Clone the repository and run Rustipo from the workspace:

```bash
git clone https://github.com/fcendesu/rustipo.git
cd rustipo
cargo run -- new my-site
cd my-site
../target/debug/rustipo dev
```

The repository also contains real example sites:

- `examples/basic-portfolio/`
- `examples/journal/`
- `examples/knowledge-base/`
- `site/` for the published docs site

## Workflow

1. Create a branch.
2. Make focused changes.
3. Run checks:

```bash
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

4. If your change affects the docs site, verify it builds:

```bash
cd site
cargo run --quiet -- build
```

5. For changes that affect packaging or release readiness, verify the published crate contents:

```bash
cargo package --allow-dirty --list
```

6. Open a pull request with a clear description.

## What makes a strong contribution

- follow existing module boundaries instead of mixing unrelated logic together
- keep behavior changes and refactors separate when practical
- prefer explicit, readable code over clever abstractions
- include at least one regression test when fixing a bug
- update docs when user-facing behavior changes

## Docs contributions

Docs improvements are welcome even when they do not change code.

Helpful places to update together:

- `README.md` for project-facing onboarding
- `docs/*.md` for repository docs
- `site/content/` for the published docs site

If you change command behavior, template context, output paths, or supported features, it is
usually worth updating both the repository docs and the docs site.

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
- Mention any docs or example sites you updated.

## Release and maintainer notes

- Do not hand-edit `CHANGELOG.md` or release version files as part of a normal feature PR.
- Maintainers use Release Please for versioning and changelog preparation.
- Maintainer-specific references:
  - [docs/release.md](./docs/release.md)
  - [docs/ci.md](./docs/ci.md)
