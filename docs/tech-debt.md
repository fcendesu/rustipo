# Technical Debt

This file tracks known implementation debt that should be addressed after the related milestone foundation exists.

## Debt register

### 1) Temporary `#[allow(dead_code)]` annotations remain in core models

- Area: `src/content/*`, `src/theme/*`, `src/render/*`
- Current state: several structs/fields still use `#[allow(dead_code)]` from early scaffolding
- Impact: can hide stale fields and reduce signal from compiler warnings
- Proposed fix:
  - audit each `allow(dead_code)` usage
  - remove allowances where fields are now consumed
  - for intentionally-unused fields, document rationale near the type
- Priority: Medium
- Target: Near-term cleanup

### 2) Frontmatter `date` remains a raw string at parse-time

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

### 3) Shortcode system is intentionally minimal and string-based

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
- Target: `0.3.0` prep

### 4) Release workflow still needs final validation after merge-strategy change

- Area: `.github/workflows/release-please.yml`, repo settings
- Current state: merge policy now preserves commits (rebase/merge-commit), but release flow should be re-verified end-to-end
- Impact: potential release friction if assumptions were based on squash merges
- Proposed fix:
  - run one full release cycle (`release PR -> merge -> tag/release`)
  - document exact maintainer release steps in docs
- Priority: Medium
- Target: Next release cycle
