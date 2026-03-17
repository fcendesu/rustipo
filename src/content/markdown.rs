use std::sync::OnceLock;
use std::collections::BTreeMap;

use pulldown_cmark::{CodeBlockKind, CowStr, Event, Options, Parser, Tag, TagEnd, html};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub fn render_html(markdown: &str) -> String {
    let markdown = preprocess_shortcodes(markdown);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(&markdown, options);
    let parser = replace_code_blocks_with_highlighted_html(parser);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

fn preprocess_shortcodes(markdown: &str) -> String {
    let mut output = String::with_capacity(markdown.len());
    let mut in_fence = false;

    for line in markdown.split_inclusive('\n') {
        if line.trim_start().starts_with("```") {
            in_fence = !in_fence;
            output.push_str(line);
            continue;
        }

        if in_fence {
            output.push_str(line);
            continue;
        }

        output.push_str(&replace_shortcodes_in_text(line));
    }

    output
}

fn replace_shortcodes_in_text(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut cursor = 0;

    while let Some(start_rel) = input[cursor..].find("{{<") {
        let start = cursor + start_rel;
        output.push_str(&input[cursor..start]);

        let Some(end_rel) = input[start + 3..].find(">}}") else {
            output.push_str(&input[start..]);
            return output;
        };
        let end = start + 3 + end_rel + 3;
        let raw = &input[start..end];
        let inner = input[start + 3..start + 3 + end_rel].trim();

        if let Some(rendered) = render_shortcode(inner) {
            output.push_str(&rendered);
        } else {
            output.push_str(raw);
        }

        cursor = end;
    }

    output.push_str(&input[cursor..]);
    output
}

fn render_shortcode(input: &str) -> Option<String> {
    let (name, attrs_raw) = split_shortcode_name_and_attrs(input)?;
    let attrs = parse_shortcode_attrs(attrs_raw)?;

    match name {
        "youtube" => {
            let id = attrs.get("id")?;
            let id = id.trim();
            if id.is_empty() {
                return None;
            }
            Some(format!(
                "<div class=\"rustipo-shortcode rustipo-youtube\"><iframe src=\"https://www.youtube.com/embed/{}\" title=\"YouTube video\" loading=\"lazy\" allowfullscreen></iframe></div>",
                escape_html(id)
            ))
        }
        "link" => {
            let href = attrs.get("href")?;
            let text = attrs.get("text").cloned().unwrap_or_else(|| href.clone());
            Some(format!(
                "<a class=\"rustipo-shortcode rustipo-link\" href=\"{}\">{}</a>",
                escape_html(href),
                escape_html(&text)
            ))
        }
        _ => None,
    }
}

fn split_shortcode_name_and_attrs(input: &str) -> Option<(&str, &str)> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    let mut split_idx = None;
    for (idx, ch) in trimmed.char_indices() {
        if ch.is_whitespace() {
            split_idx = Some(idx);
            break;
        }
    }

    match split_idx {
        Some(idx) => Some((&trimmed[..idx], trimmed[idx..].trim())),
        None => Some((trimmed, "")),
    }
}

fn parse_shortcode_attrs(input: &str) -> Option<BTreeMap<String, String>> {
    let mut attrs = BTreeMap::new();
    let mut index = 0;
    let bytes = input.as_bytes();

    while index < bytes.len() {
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        if index >= bytes.len() {
            break;
        }

        let key_start = index;
        while index < bytes.len()
            && (bytes[index].is_ascii_alphanumeric()
                || bytes[index] == b'_'
                || bytes[index] == b'-')
        {
            index += 1;
        }
        if key_start == index {
            return None;
        }
        let key = &input[key_start..index];

        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        if index >= bytes.len() || bytes[index] != b'=' {
            return None;
        }
        index += 1;

        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        if index >= bytes.len() || bytes[index] != b'"' {
            return None;
        }
        index += 1;

        let value_start = index;
        while index < bytes.len() && bytes[index] != b'"' {
            index += 1;
        }
        if index >= bytes.len() {
            return None;
        }
        let value = &input[value_start..index];
        index += 1;

        attrs.insert(key.to_string(), value.to_string());
    }

    Some(attrs)
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

    #[test]
    fn renders_youtube_shortcode() {
        let html = render_html("{{< youtube id=\"dQw4w9WgXcQ\" >}}");
        assert!(html.contains("youtube.com/embed/dQw4w9WgXcQ"));
        assert!(html.contains("rustipo-youtube"));
    }

    #[test]
    fn renders_link_shortcode() {
        let html = render_html("{{< link href=\"https://example.com\" text=\"Visit\" >}}");
        assert!(html.contains("href=\"https://example.com\""));
        assert!(html.contains(">Visit<"));
    }

    #[test]
    fn leaves_unknown_shortcode_as_text() {
        let html = render_html("{{< unknown foo=\"bar\" >}}");
        assert!(html.contains("unknown"));
        assert!(html.contains("foo"));
        assert!(html.contains("bar"));
    }

    #[test]
    fn does_not_render_shortcode_inside_code_fence() {
        let html = render_html("```\n{{< youtube id=\"dQw4w9WgXcQ\" >}}\n```");
        assert!(!html.contains("youtube.com/embed/"));
    }
}
