# Technical Debt

This file tracks known implementation debt that should be addressed after the related milestone foundation exists.

## Debt register

- No active items right now.

## Recently resolved

- Frontmatter date typing/validation:
  - introduced shared `ContentDate` wrapper with strict `YYYY-MM-DD` validation
  - moved validation to frontmatter decode stage (invalid dates now fail fast)
  - simplified RSS/archive consumers to use validated date data
- Shortcode modularization:
  - extracted shortcode preprocessing from `src/content/markdown.rs` into `src/content/shortcodes/`
  - split parser and renderer responsibilities into separate modules
  - preserved current shortcode behavior while reducing markdown module complexity
- Dead code cleanup:
  - removed blanket `#[allow(dead_code)]` annotations
  - kept only targeted allowances with explicit rationale for contract fields
- Runtime code highlighting panic:
  - replaced `expect(...)` with a safe fallback path in `src/content/markdown.rs`
  - added tests for empty/default theme-set behavior
- Release workflow validation after merge-strategy change:
  - completed end-to-end on 2026-03-17 with successful Release Please run
  - release flow has continued successfully through `rustipo-v0.3.1`
