# Technical Debt

This file tracks known implementation debt that should be addressed after the related milestone foundation exists.

## Debt register

### 1) Frontmatter `date` remains a raw string at parse-time

- Area: `src/content/frontmatter.rs` and downstream output modules
- Current state: `date` is parsed as `Option<String>` and validated only in specific pipelines (RSS/archive)
- Impact:
  - invalid dates can survive into page data
  - behavior differs by feature (RSS may skip, other outputs may still render)
- Proposed fix:
  - introduce a shared date parsing utility or typed date wrapper
  - validate at content-model build stage with readable file/field errors
  - decide policy for invalid dates (fail-fast vs warn-and-skip)
- Priority: High
- Target: Next quality pass

### 2) Shortcode system is intentionally minimal and string-based

- Area: `src/content/markdown.rs`
- Current state: shortcodes are preprocessed with lightweight parsing and two built-ins (`youtube`, `link`)
- Impact:
  - no reusable shortcode registry yet
  - limited validation/escaping policy per shortcode
  - adding many shortcodes will bloat current module
- Proposed fix:
  - extract shortcode parser/registry into `src/content/shortcodes/`
  - define typed argument decoding and shared error policy
  - add integration tests for nested/edge-case shortcode scenarios
- Priority: Medium
- Target: Next content pipeline refactor

## Recently resolved

- Dead code cleanup:
  - removed blanket `#[allow(dead_code)]` annotations
  - kept only targeted allowances with explicit rationale for contract fields
- Runtime code highlighting panic:
  - replaced `expect(...)` with a safe fallback path in `src/content/markdown.rs`
  - added tests for empty/default theme-set behavior
- Release workflow validation after merge-strategy change:
  - completed end-to-end on 2026-03-17 with successful Release Please run
  - `rustipo-v0.3.0` tag/release published
