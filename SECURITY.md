# Security Policy

Rustipo is distributed as an open-source static site generator through crates.io and GitHub
release binaries. If you believe you have found a security issue, please report it privately
instead of opening a public GitHub issue first.

## Reporting A Vulnerability

Please contact the maintainer privately through [GitHub](https://github.com/fcendesu).

When possible, include:

- a short description of the issue
- the affected Rustipo version
- steps to reproduce or a proof of concept
- potential impact
- any suggested mitigation or patch ideas

## Public Issues

For suspected security problems, avoid filing a normal public issue before the maintainer has had a
chance to review it privately.

Public issues are still fine for:

- general hardening ideas
- dependency update suggestions
- non-sensitive maintenance work

## Scope

Useful reports usually involve things like:

- unsafe generated output that could put site owners or readers at risk
- release or packaging issues that could affect distributed binaries or crates
- security-sensitive workflow or deployment behavior

## Response Expectations

Rustipo is a small maintained project, so response times may vary. Reports will be reviewed in good
faith and handled as carefully as possible.
