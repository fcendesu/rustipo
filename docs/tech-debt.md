# Technical Debt

This file tracks known implementation debt that should be addressed after the related milestone foundation exists.

## Debt register

### 1) Build command still does not write output files

- Area: `build` pipeline
- Current state: pages are parsed and rendered in-memory but not written to `dist/`
- Impact: main user-facing build result is incomplete
- Proposed fix: add output writer for pretty URLs and HTML file emission
- Target milestone: Milestone 4

### 2) Theme section template is validated but not used yet

- Area: template rendering
- Current state: `section.html` is required in theme contract but not rendered in pipeline
- Impact: blog/project index pages are missing
- Proposed fix: generate section pages (`/blog/`, `/projects/`) and render with `section.html`
- Target milestone: Milestone 4

### 3) Asset copy behavior not implemented

- Area: output/assets
- Current state: user `static/` and theme `static/` are not copied to build output
- Impact: rendered pages may reference missing assets
- Proposed fix: implement asset copy and collision handling policy
- Target milestone: Milestone 4

### 4) Temporary `#[allow(dead_code)]` annotations

- Area: content/theme models
- Current state: some structs/fields are allowed as dead code during incremental implementation
- Impact: can hide stale or unused fields if left in place
- Proposed fix: remove allowances when all fields are consumed by final pipeline
- Target milestone: Milestone 4-5

### 5) Frontmatter date is currently an unvalidated string

- Area: frontmatter model
- Current state: `date` parsed as `Option<String>`
- Impact: invalid dates are not caught early
- Proposed fix: add date parsing/validation strategy with readable errors
- Target milestone: Post-MVP or Milestone 5

