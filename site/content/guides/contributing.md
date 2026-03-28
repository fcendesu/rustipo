---
title: Contributing
summary: Learn how to make focused contributions to Rustipo across code, docs, examples, and themes.
order: 4
---

# Contributing

Rustipo is a compact open-source project, so focused improvements go a long way. Contributions are
welcome across:

- core behavior
- documentation
- example sites
- themes and palettes
- tests and regression coverage

## Best First Reads

If you are new to the repository, these pages help you orient quickly:

- [README on GitHub](https://github.com/fcendesu/rustipo/blob/master/README.md)
- [Contributing guide on GitHub](https://github.com/fcendesu/rustipo/blob/master/CONTRIBUTING.md)
- [Code of Conduct on GitHub](https://github.com/fcendesu/rustipo/blob/master/CODE_OF_CONDUCT.md)
- [Roadmap](/roadmap/)
- [Project structure on GitHub](https://github.com/fcendesu/rustipo/blob/master/docs/project-structure.md)
- [Themes and palettes](/reference/themes-and-palettes/)

## Local Setup

Clone the repository and run Rustipo from the workspace:

```bash
git clone https://github.com/fcendesu/rustipo.git
cd rustipo
cargo run -- new my-site
cd my-site
../target/debug/rustipo dev
```

That gives you a quick way to verify the tool locally while still working against the repository
version.

## Useful In-Repo Projects

Rustipo includes a few real projects you can use while changing behavior:

- `examples/basic-portfolio/` for the starter personal-site shape
- `examples/journal/` for a blog-focused editorial shape
- `examples/knowledge-base/` for a docs-and-notes shape
- `site/` for the published documentation site itself

## Recommended Contributor Loop

From the repository root:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test -q
```

If your change affects the docs site, also verify:

```bash
cd site
cargo run --quiet -- build
```

If your change affects packaging or release readiness, verify crate contents too:

```bash
cargo package --allow-dirty --list
```

## What Makes A Strong PR

- keep the change focused and reviewable
- keep behavior changes and refactors separate when practical
- add or update tests for changed behavior
- update docs when user-facing behavior changes
- explain what changed and why in the PR description

Rustipo uses Conventional Commits:

```text
<type>(<scope>): <subject>
```

Examples:

- `feat(content): add shortcode asset collection`
- `fix(output): honor base_url subpaths`
- `docs(repo): improve contributor onboarding`

## Docs Contributions

Docs-only changes are welcome.

When behavior changes, it is usually worth updating both:

- repository docs under `docs/`
- the published docs site under `site/content/`

That keeps GitHub readers and docs-site readers on the same page.

## Maintainer Notes

Normal contribution PRs should not hand-edit release version files or `CHANGELOG.md`.

Maintainers handle release preparation with Release Please. If you need the maintainer-specific
workflow, see:

- [Release and publish workflow](https://github.com/fcendesu/rustipo/blob/master/docs/release.md)
- [CI notes](https://github.com/fcendesu/rustipo/blob/master/docs/ci.md)

## Related Pages

- [Getting started](/guides/getting-started/)
- [Build the docs site](/guides/building-the-docs-site/)
- [Roadmap](/roadmap/)
