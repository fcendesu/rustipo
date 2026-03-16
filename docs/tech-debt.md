# Technical Debt

This file tracks known implementation debt that should be addressed after the related milestone foundation exists.

## Debt register

### 1) Temporary `#[allow(dead_code)]` annotations

- Area: content/theme models
- Current state: some structs/fields are allowed as dead code during incremental implementation
- Impact: can hide stale or unused fields if left in place
- Proposed fix: remove allowances when all fields are consumed by final pipeline
- Target milestone: Milestone 4-5

### 2) Frontmatter date is currently an unvalidated string

- Area: frontmatter model
- Current state: `date` parsed as `Option<String>`
- Impact: invalid dates are not caught early
- Proposed fix: add date parsing/validation strategy with readable errors
- Target milestone: Post-MVP or Milestone 5

### 3) Serve command address is currently fixed

- Area: CLI UX / server
- Current state: `rustipo serve` is hardcoded to `127.0.0.1:3000`
- Impact: no override for occupied port or custom host/port needs
- Proposed fix: add serve flags (for example `--host` and `--port`)
- Target milestone: Post-MVP or Milestone 6
