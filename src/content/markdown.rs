use std::sync::OnceLock;

use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd, html};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub fn render_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(markdown, options);
    let parser = replace_code_blocks_with_highlighted_html(parser);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

fn replace_code_blocks_with_highlighted_html<'a>(
    parser: Parser<'a>,
) -> impl Iterator<Item = Event<'a>> {
    let mut events = Vec::new();
    let mut in_code_block = false;
    let mut code_language: Option<String> = None;
    let mut code_content = String::new();

    for event in parser {
        if in_code_block {
            match event {
                Event::End(TagEnd::CodeBlock) => {
                    let highlighted = highlight_code(&code_content, code_language.as_deref());
                    events.push(Event::Html(CowStr::Boxed(highlighted.into_boxed_str())));
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

    events.into_iter()
}

fn highlight_code(code: &str, language: Option<&str>) -> String {
    let syntax_set = syntax_set();
    let theme_set = theme_set();
    let theme = theme_set
        .themes
        .get("base16-ocean.dark")
        .or_else(|| theme_set.themes.values().next())
        .expect("syntect default themes must contain at least one theme");
    let syntax = language
        .and_then(|lang| syntax_set.find_syntax_by_token(lang))
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

    highlighted_html_for_string(code, syntax_set, syntax, theme)
        .unwrap_or_else(|_| fallback_code_html(code))
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

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::render_html;

    #[test]
    fn renders_basic_markdown() {
        let html = render_html("# Hello\n\nThis is **Rustipo**.");
        assert!(html.contains("<h1>Hello</h1>"));
        assert!(html.contains("<strong>Rustipo</strong>"));
    }

    #[test]
    fn renders_highlighted_code_block() {
        let html = render_html("```rust\nfn main() {}\n```");
        assert!(html.contains("<pre"));
        assert!(html.contains("<span"));
    }
}
