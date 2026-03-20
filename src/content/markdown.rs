use std::sync::OnceLock;

use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd, html};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedMarkdown {
    pub html: String,
    pub has_mermaid: bool,
}

pub fn render_html(markdown: &str) -> RenderedMarkdown {
    let markdown = crate::content::shortcodes::preprocess(markdown);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(&markdown, options);
    let rendered = replace_code_blocks_with_highlighted_html(parser);
    let mut output = String::new();
    html::push_html(&mut output, rendered.events.into_iter());
    RenderedMarkdown {
        html: output,
        has_mermaid: rendered.has_mermaid,
    }
}

struct CodeBlockRenderResult<'a> {
    events: Vec<Event<'a>>,
    has_mermaid: bool,
}

fn replace_code_blocks_with_highlighted_html<'a>(parser: Parser<'a>) -> CodeBlockRenderResult<'a> {
    let mut events = Vec::new();
    let mut in_code_block = false;
    let mut code_language: Option<String> = None;
    let mut code_content = String::new();
    let mut has_mermaid = false;

    for event in parser {
        if in_code_block {
            match event {
                Event::End(TagEnd::CodeBlock) => {
                    let rendered = if code_language.as_deref() == Some("mermaid") {
                        has_mermaid = true;
                        render_mermaid_block(&code_content)
                    } else {
                        highlight_code(&code_content, code_language.as_deref())
                    };
                    events.push(Event::Html(CowStr::Boxed(rendered.into_boxed_str())));
                    in_code_block = false;
                    code_language = None;
                    code_content.clear();
                }
                Event::Text(text) | Event::Code(text) => code_content.push_str(&text),
                Event::SoftBreak | Event::HardBreak => code_content.push('\n'),
                _ => {}
            }
            continue;
        }

        match event {
            Event::Start(Tag::CodeBlock(kind)) => {
                in_code_block = true;
                code_language = match kind {
                    CodeBlockKind::Fenced(lang) => {
                        let language = lang.trim().to_string();
                        if language.is_empty() {
                            None
                        } else {
                            Some(language)
                        }
                    }
                    CodeBlockKind::Indented => None,
                };
            }
            _ => events.push(event),
        }
    }

    CodeBlockRenderResult {
        events,
        has_mermaid,
    }
}

fn highlight_code(code: &str, language: Option<&str>) -> String {
    let syntax_set = syntax_set();
    let theme_set = theme_set();
    let Some(theme) = preferred_theme(theme_set) else {
        return fallback_code_html(code);
    };
    let syntax = language
        .and_then(|lang| syntax_set.find_syntax_by_token(lang))
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

    highlighted_html_for_string(code, syntax_set, syntax, theme)
        .unwrap_or_else(|_| fallback_code_html(code))
}

fn preferred_theme(theme_set: &ThemeSet) -> Option<&syntect::highlighting::Theme> {
    theme_set
        .themes
        .get("base16-ocean.dark")
        .or_else(|| theme_set.themes.values().next())
}

fn syntax_set() -> &'static SyntaxSet {
    static SYNTAX_SET: OnceLock<SyntaxSet> = OnceLock::new();
    SYNTAX_SET.get_or_init(SyntaxSet::load_defaults_newlines)
}

fn theme_set() -> &'static ThemeSet {
    static THEME_SET: OnceLock<ThemeSet> = OnceLock::new();
    THEME_SET.get_or_init(ThemeSet::load_defaults)
}

fn fallback_code_html(code: &str) -> String {
    format!("<pre><code>{}</code></pre>", escape_html(code))
}

fn render_mermaid_block(code: &str) -> String {
    format!("<pre class=\"mermaid\">{}</pre>", escape_html(code))
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use syntect::highlighting::ThemeSet;

    use super::{preferred_theme, render_html};

    #[test]
    fn renders_basic_markdown() {
        let rendered = render_html("# Hello\n\nThis is **Rustipo**.");
        assert!(rendered.html.contains("<h1>Hello</h1>"));
        assert!(rendered.html.contains("<strong>Rustipo</strong>"));
        assert!(!rendered.has_mermaid);
    }

    #[test]
    fn renders_highlighted_code_block() {
        let rendered = render_html("```rust\nfn main() {}\n```");
        assert!(rendered.html.contains("<pre"));
        assert!(rendered.html.contains("<span"));
        assert!(!rendered.has_mermaid);
    }

    #[test]
    fn renders_mermaid_code_block_without_highlighting() {
        let rendered = render_html("```mermaid\ngraph TD\n  A --> B\n```");
        assert!(rendered.has_mermaid);
        assert!(rendered.html.contains("<pre class=\"mermaid\">"));
        assert!(rendered.html.contains("graph TD"));
        assert!(!rendered.html.contains("<span"));
    }

    #[test]
    fn escapes_html_inside_mermaid_code_block() {
        let rendered = render_html("```mermaid\ngraph TD\n  A[<b>x</b>] --> B\n```");
        assert!(rendered.html.contains("&lt;b&gt;x&lt;/b&gt;"));
    }

    #[test]
    fn preferred_theme_returns_none_for_empty_theme_set() {
        let theme_set = ThemeSet::default();
        assert!(preferred_theme(&theme_set).is_none());
    }

    #[test]
    fn preferred_theme_returns_some_for_default_themes() {
        let theme_set = ThemeSet::load_defaults();
        assert!(preferred_theme(&theme_set).is_some());
    }
}
