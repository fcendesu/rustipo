use crate::palette::models::Palette;

#[derive(Clone, Copy)]
pub struct SemanticPaletteToken {
    pub css_name: &'static str,
    pub value: fn(&Palette) -> &str,
}

pub const STABLE_SEMANTIC_TOKENS: [SemanticPaletteToken; 10] = [
    SemanticPaletteToken {
        css_name: "rustipo-bg",
        value: |palette| palette.bg.as_str(),
    },
    SemanticPaletteToken {
        css_name: "rustipo-text",
        value: |palette| palette.text.as_str(),
    },
    SemanticPaletteToken {
        css_name: "rustipo-surface-muted",
        value: |palette| palette.surface_muted.as_str(),
    },
    SemanticPaletteToken {
        css_name: "rustipo-border",
        value: |palette| palette.border.as_str(),
    },
    SemanticPaletteToken {
        css_name: "rustipo-blockquote-border",
        value: |palette| palette.blockquote_border.as_str(),
    },
    SemanticPaletteToken {
        css_name: "rustipo-link",
        value: |palette| palette.link.as_str(),
    },
    SemanticPaletteToken {
        css_name: "rustipo-link-hover",
        value: |palette| palette.link_hover.as_str(),
    },
    SemanticPaletteToken {
        css_name: "rustipo-code-bg",
        value: |palette| palette.code_bg.as_str(),
    },
    SemanticPaletteToken {
        css_name: "rustipo-code-text",
        value: |palette| palette.code_text.as_str(),
    },
    SemanticPaletteToken {
        css_name: "rustipo-table-header-bg",
        value: |palette| palette.table_header_bg.as_str(),
    },
];

#[derive(Clone, Copy)]
pub struct CanonicalPaletteToken {
    pub css_name: &'static str,
    pub preferred_extra_tokens: &'static [&'static str],
    pub fallback: fn(&Palette) -> &str,
}

pub const CANONICAL_RICH_TOKENS: [CanonicalPaletteToken; 16] = [
    CanonicalPaletteToken {
        css_name: "rustipo-base",
        preferred_extra_tokens: &["base"],
        fallback: |palette| palette.bg.as_str(),
    },
    CanonicalPaletteToken {
        css_name: "rustipo-mantle",
        preferred_extra_tokens: &["mantle"],
        fallback: |palette| palette.surface_muted.as_str(),
    },
    CanonicalPaletteToken {
        css_name: "rustipo-crust",
        preferred_extra_tokens: &["crust"],
        fallback: |palette| palette.code_bg.as_str(),
    },
    CanonicalPaletteToken {
        css_name: "rustipo-surface-0",
        preferred_extra_tokens: &["surface0"],
        fallback: |palette| palette.surface_muted.as_str(),
    },
    CanonicalPaletteToken {
        css_name: "rustipo-surface-1",
        preferred_extra_tokens: &["surface1"],
        fallback: |palette| palette.table_header_bg.as_str(),
    },
    CanonicalPaletteToken {
        css_name: "rustipo-surface-2",
        preferred_extra_tokens: &["surface2"],
        fallback: |palette| palette.border.as_str(),
    },
    CanonicalPaletteToken {
        css_name: "rustipo-overlay-0",
        preferred_extra_tokens: &["overlay0"],
        fallback: |palette| palette.border.as_str(),
    },
    CanonicalPaletteToken {
        css_name: "rustipo-overlay-1",
        preferred_extra_tokens: &["overlay1"],
        fallback: |palette| palette.code_text.as_str(),
    },
    CanonicalPaletteToken {
        css_name: "rustipo-overlay-2",
        preferred_extra_tokens: &["overlay2"],
        fallback: |palette| palette.code_text.as_str(),
    },
    CanonicalPaletteToken {
        css_name: "rustipo-subtext-0",
        preferred_extra_tokens: &["subtext0"],
        fallback: |palette| palette.code_text.as_str(),
    },
    CanonicalPaletteToken {
        css_name: "rustipo-subtext-1",
        preferred_extra_tokens: &["subtext1"],
        fallback: |palette| palette.code_text.as_str(),
    },
    CanonicalPaletteToken {
        css_name: "rustipo-accent",
        preferred_extra_tokens: &["accent", "blue"],
        fallback: |palette| palette.link.as_str(),
    },
    CanonicalPaletteToken {
        css_name: "rustipo-accent-strong",
        preferred_extra_tokens: &["accent-strong", "mauve", "lavender"],
        fallback: |palette| palette.link_hover.as_str(),
    },
    CanonicalPaletteToken {
        css_name: "rustipo-success",
        preferred_extra_tokens: &["success", "green"],
        fallback: |palette| palette.link.as_str(),
    },
    CanonicalPaletteToken {
        css_name: "rustipo-warning",
        preferred_extra_tokens: &["warning", "yellow"],
        fallback: |palette| palette.link_hover.as_str(),
    },
    CanonicalPaletteToken {
        css_name: "rustipo-danger",
        preferred_extra_tokens: &["danger", "red"],
        fallback: |palette| palette.blockquote_border.as_str(),
    },
];

pub fn canonical_token_value<'a>(palette: &'a Palette, token: &CanonicalPaletteToken) -> &'a str {
    for name in token.preferred_extra_tokens {
        if let Some(value) = palette.extra_tokens.get(*name) {
            return value;
        }
    }

    (token.fallback)(palette)
}
