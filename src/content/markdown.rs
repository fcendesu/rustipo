use std::collections::BTreeMap;
use std::sync::OnceLock;

use pulldown_cmark::{
    CodeBlockKind, CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd, html,
};
use syntect::highlighting::ThemeSet;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

use crate::content::toc::{
    FlatTocItem, TocItem, build_nested_toc, normalize_heading_title, unique_heading_id,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RenderedMarkdown {
    pub html: String,
    pub has_mermaid: bool,
    pub has_math: bool,
    pub toc: Vec<TocItem>,
}

pub fn render_html(markdown: &str) -> RenderedMarkdown {
    let markdown = crate::content::shortcodes::preprocess(markdown);

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_MATH);
    options.insert(Options::ENABLE_GFM);

    let parser = Parser::new_ext(&markdown, options);
    let rendered = replace_code_blocks_with_highlighted_html(parser);
    let mut output = String::new();
    html::push_html(&mut output, rendered.events.into_iter());
    RenderedMarkdown {
        html: output,
        has_mermaid: rendered.has_mermaid,
        has_math: rendered.has_math,
        toc: rendered.toc,
    }
}

struct CodeBlockRenderResult<'a> {
    events: Vec<Event<'a>>,
    has_mermaid: bool,
    has_math: bool,
    toc: Vec<TocItem>,
}

struct PendingHeading<'a> {
    level: HeadingLevel,
    events: Vec<Event<'a>>,
    plain_text: String,
}

impl<'a> PendingHeading<'a> {
    fn new(level: HeadingLevel) -> Self {
        Self {
            level,
            events: Vec::new(),
            plain_text: String::new(),
        }
    }
}

struct PendingParagraph<'a> {
    events: Vec<Event<'a>>,
}

impl<'a> PendingParagraph<'a> {
    fn new() -> Self {
        Self { events: Vec::new() }
    }
}

fn replace_code_blocks_with_highlighted_html<'a>(parser: Parser<'a>) -> CodeBlockRenderResult<'a> {
    let mut events = Vec::new();
    let mut in_code_block = false;
    let mut code_language: Option<String> = None;
    let mut code_content = String::new();
    let mut has_mermaid = false;
    let mut has_math = false;
    let mut pending_heading: Option<PendingHeading<'a>> = None;
    let mut pending_paragraph: Option<PendingParagraph<'a>> = None;
    let mut heading_ids = BTreeMap::new();
    let mut toc_entries = Vec::new();

    for event in parser {
        if let Some(heading) = pending_heading.as_mut() {
            match event {
                Event::End(TagEnd::Heading(_)) => {
                    let heading = pending_heading
                        .take()
                        .expect("pending heading should exist");
                    let rendered = render_heading(heading, &mut heading_ids);
                    toc_entries.push(rendered.toc_item);
                    events.push(Event::Html(CowStr::Boxed(rendered.html.into_boxed_str())));
                }
                _ => {
                    append_heading_text(&mut heading.plain_text, &event);
                    heading.events.push(event);
                }
            }
            continue;
        }

        if let Some(paragraph) = pending_paragraph.as_mut() {
            match event {
                Event::End(TagEnd::Paragraph) => {
                    let paragraph = pending_paragraph
                        .take()
                        .expect("pending paragraph should exist");
                    events.push(Event::Html(CowStr::Boxed(
                        render_paragraph(paragraph.events).into_boxed_str(),
                    )));
                }
                Event::InlineMath(_) | Event::DisplayMath(_) => {
                    has_math = true;
                    paragraph.events.push(event);
                }
                _ => paragraph.events.push(event),
            }
            continue;
        }

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
            Event::Start(Tag::Heading { level, .. }) => {
                pending_heading = Some(PendingHeading::new(level));
            }
            Event::Start(Tag::Paragraph) => {
                pending_paragraph = Some(PendingParagraph::new());
            }
            Event::InlineMath(_) | Event::DisplayMath(_) => {
                has_math = true;
                events.push(event);
            }
            _ => events.push(event),
        }
    }

    CodeBlockRenderResult {
        events,
        has_mermaid,
        has_math,
        toc: build_nested_toc(toc_entries),
    }
}

fn render_paragraph<'a>(paragraph_events: Vec<Event<'a>>) -> String {
    try_render_standalone_image(&paragraph_events).unwrap_or_else(|| {
        let mut html_output = String::new();
        let mut events = Vec::with_capacity(paragraph_events.len() + 2);
        events.push(Event::Start(Tag::Paragraph));
        events.extend(paragraph_events);
        events.push(Event::End(TagEnd::Paragraph));
        html::push_html(&mut html_output, events.into_iter());
        html_output
    })
}

fn try_render_standalone_image(paragraph_events: &[Event<'_>]) -> Option<String> {
    let mut events = paragraph_events.iter();
    let first = events.next()?;
    let Event::Start(Tag::Image {
        dest_url, title, ..
    }) = first
    else {
        return None;
    };

    let mut alt = String::new();
    loop {
        match events.next()? {
            Event::End(TagEnd::Image) => break,
            Event::Text(text)
            | Event::Code(text)
            | Event::InlineMath(text)
            | Event::DisplayMath(text)
            | Event::FootnoteReference(text) => alt.push_str(text),
            Event::SoftBreak | Event::HardBreak => alt.push(' '),
            Event::Start(_) | Event::End(_) => {}
            _ => return None,
        }
    }

    if events.next().is_some() {
        return None;
    }

    let title = parse_image_title(title.as_ref());
    let mut classes = vec!["markdown-image"];
    if let Some(size) = title.size_class() {
        classes.push(size);
    }
    if let Some(alignment) = title.align_class() {
        classes.push(alignment);
    }

    let escaped_src = escape_html_attr(dest_url.as_ref());
    let escaped_alt = escape_html_attr(alt.trim());
    let mut html_output = format!(
        "<figure class=\"{}\"><img class=\"markdown-image-img\" src=\"{}\" alt=\"{}\" loading=\"lazy\" decoding=\"async\" />",
        classes.join(" "),
        escaped_src,
        escaped_alt,
    );

    if let Some(caption) = title.caption {
        html_output.push_str(&format!(
            "<figcaption class=\"markdown-image-caption\">{}</figcaption>",
            escape_html(&caption)
        ));
    }

    html_output.push_str("</figure>");
    Some(html_output)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedImageTitle {
    caption: Option<String>,
    alignment: Option<ImageAlignment>,
    size: Option<ImageSize>,
}

impl ParsedImageTitle {
    fn align_class(&self) -> Option<&'static str> {
        self.alignment.map(|alignment| match alignment {
            ImageAlignment::Left => "markdown-image-left",
            ImageAlignment::Center => "markdown-image-center",
            ImageAlignment::Right => "markdown-image-right",
        })
    }

    fn size_class(&self) -> Option<&'static str> {
        self.size.map(|size| match size {
            ImageSize::Wide => "markdown-image-wide",
            ImageSize::Full => "markdown-image-full",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ImageAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ImageSize {
    Wide,
    Full,
}

fn parse_image_title(title: &str) -> ParsedImageTitle {
    let trimmed = title.trim();
    if trimmed.is_empty() {
        return ParsedImageTitle {
            caption: None,
            alignment: None,
            size: None,
        };
    }

    let Some(directive_end) = trimmed.strip_prefix('{').and_then(|rest| rest.find('}')) else {
        return ParsedImageTitle {
            caption: Some(trimmed.to_string()),
            alignment: None,
            size: None,
        };
    };

    let directives = &trimmed[1..=directive_end];
    let mut alignment = None;
    let mut size = None;

    for token in directives
        .trim_end_matches('}')
        .split(|ch: char| ch == ',' || ch.is_ascii_whitespace())
        .filter(|token| !token.is_empty())
    {
        match token {
            "left" => alignment = Some(ImageAlignment::Left),
            "center" => alignment = Some(ImageAlignment::Center),
            "right" => alignment = Some(ImageAlignment::Right),
            "wide" => size = Some(ImageSize::Wide),
            "full" => size = Some(ImageSize::Full),
            _ => {
                return ParsedImageTitle {
                    caption: Some(trimmed.to_string()),
                    alignment: None,
                    size: None,
                };
            }
        }
    }

    let caption = trimmed[directive_end + 2..].trim();
    ParsedImageTitle {
        caption: (!caption.is_empty()).then(|| caption.to_string()),
        alignment,
        size,
    }
}

struct RenderedHeading {
    html: String,
    toc_item: FlatTocItem,
}

fn render_heading(
    heading: PendingHeading<'_>,
    heading_ids: &mut BTreeMap<String, usize>,
) -> RenderedHeading {
    let title = normalize_heading_title(&heading.plain_text);
    let id = unique_heading_id(&title, heading_ids);
    let level = heading_level_number(heading.level);
    let title = if title.is_empty() { id.clone() } else { title };

    let mut inner_html = String::new();
    html::push_html(&mut inner_html, heading.events.into_iter());

    RenderedHeading {
        html: format!("<h{level} id=\"{id}\">{inner_html}</h{level}>"),
        toc_item: FlatTocItem { level, id, title },
    }
}

fn append_heading_text(buffer: &mut String, event: &Event<'_>) {
    match event {
        Event::Text(text)
        | Event::Code(text)
        | Event::InlineMath(text)
        | Event::DisplayMath(text)
        | Event::FootnoteReference(text) => buffer.push_str(text),
        Event::SoftBreak | Event::HardBreak => buffer.push(' '),
        _ => {}
    }
}

fn heading_level_number(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
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

fn escape_html_attr(input: &str) -> String {
    escape_html(input).replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use syntect::highlighting::ThemeSet;

    use super::{preferred_theme, render_html};

    #[test]
    fn renders_basic_markdown() {
        let rendered = render_html("# Hello\n\nThis is **Rustipo**.");
        assert!(rendered.html.contains("<h1 id=\"hello\">Hello</h1>"));
        assert!(rendered.html.contains("<strong>Rustipo</strong>"));
        assert!(!rendered.has_mermaid);
        assert!(!rendered.has_math);
        assert_eq!(rendered.toc.len(), 1);
        assert_eq!(rendered.toc[0].id, "hello");
        assert_eq!(rendered.toc[0].title, "Hello");
    }

    #[test]
    fn renders_highlighted_code_block() {
        let rendered = render_html("```rust\nfn main() {}\n```");
        assert!(rendered.html.contains("<pre"));
        assert!(rendered.html.contains("<span"));
        assert!(!rendered.has_mermaid);
        assert!(!rendered.has_math);
    }

    #[test]
    fn renders_mermaid_code_block_without_highlighting() {
        let rendered = render_html("```mermaid\ngraph TD\n  A --> B\n```");
        assert!(rendered.has_mermaid);
        assert!(rendered.html.contains("<pre class=\"mermaid\">"));
        assert!(rendered.html.contains("graph TD"));
        assert!(!rendered.html.contains("<span"));
        assert!(!rendered.has_math);
    }

    #[test]
    fn escapes_html_inside_mermaid_code_block() {
        let rendered = render_html("```mermaid\ngraph TD\n  A[<b>x</b>] --> B\n```");
        assert!(rendered.html.contains("&lt;b&gt;x&lt;/b&gt;"));
    }

    #[test]
    fn renders_inline_and_display_math() {
        let rendered = render_html("Inline $a^2 + b^2$.\n\n$$c^2$$");

        assert!(rendered.has_math);
        assert!(
            rendered
                .html
                .contains("<span class=\"math math-inline\">a^2 + b^2</span>")
        );
        assert!(
            rendered
                .html
                .contains("<span class=\"math math-display\">c^2</span>")
        );
    }

    #[test]
    fn renders_supported_alert_blockquote() {
        let rendered = render_html("> [!NOTE]\n> Hello **world**");

        assert!(
            rendered
                .html
                .contains("<blockquote class=\"markdown-alert-note\">")
        );
        assert!(
            rendered
                .html
                .contains("<p>Hello <strong>world</strong></p>")
        );
        assert!(!rendered.html.contains("[!NOTE]"));
    }

    #[test]
    fn unsupported_alert_variant_degrades_to_plain_blockquote() {
        let rendered = render_html("> [!DANGER]\n> Heads up");

        assert!(rendered.html.contains("<blockquote>"));
        assert!(rendered.html.contains("[!DANGER]\nHeads up"));
        assert!(!rendered.html.contains("markdown-alert-"));
    }

    #[test]
    fn renders_standalone_image_as_figure_with_caption() {
        let rendered = render_html("![A lighthouse](/img/lighthouse.jpg \"Lighthouse at dusk\")");

        assert!(rendered.html.contains("<figure class=\"markdown-image\">"));
        assert!(rendered.html.contains(
            "src=\"/img/lighthouse.jpg\" alt=\"A lighthouse\" loading=\"lazy\" decoding=\"async\""
        ));
        assert!(rendered.html.contains(
            "<figcaption class=\"markdown-image-caption\">Lighthouse at dusk</figcaption>"
        ));
        assert!(!rendered.html.contains("<p><img"));
    }

    #[test]
    fn supports_standalone_image_size_and_alignment_directives() {
        let rendered = render_html("![JPEG flow](/img/jpeg.png \"{wide right} JPEG pipeline\")");

        assert!(rendered.html.contains(
            "<figure class=\"markdown-image markdown-image-wide markdown-image-right\">"
        ));
        assert!(
            rendered.html.contains(
                "<figcaption class=\"markdown-image-caption\">JPEG pipeline</figcaption>"
            )
        );
    }

    #[test]
    fn unknown_image_directive_degrades_to_plain_caption() {
        let rendered = render_html("![Diagram](/img/diagram.png \"{giant} Diagram overview\")");

        assert!(rendered.html.contains("<figure class=\"markdown-image\">"));
        assert!(rendered.html.contains(
            "<figcaption class=\"markdown-image-caption\">{giant} Diagram overview</figcaption>"
        ));
        assert!(!rendered.html.contains("markdown-image-wide"));
        assert!(!rendered.html.contains("markdown-image-full"));
    }

    #[test]
    fn keeps_inline_images_inside_paragraphs() {
        let rendered = render_html("Look at ![this chart](/img/chart.png) here.");

        assert!(
            rendered
                .html
                .contains("<p>Look at <img src=\"/img/chart.png\" alt=\"this chart\" /> here.</p>")
        );
        assert!(!rendered.html.contains("<figure class=\"markdown-image\">"));
    }

    #[test]
    fn builds_nested_toc_and_heading_ids() {
        let rendered = render_html("# Intro\n\n## Install\n\n### Cargo\n\n## Next");

        assert!(rendered.html.contains("<h1 id=\"intro\">Intro</h1>"));
        assert!(rendered.html.contains("<h2 id=\"install\">Install</h2>"));
        assert!(rendered.html.contains("<h3 id=\"cargo\">Cargo</h3>"));
        assert!(rendered.html.contains("<h2 id=\"next\">Next</h2>"));

        assert_eq!(rendered.toc.len(), 1);
        assert_eq!(rendered.toc[0].title, "Intro");
        assert_eq!(rendered.toc[0].children.len(), 2);
        assert_eq!(rendered.toc[0].children[0].title, "Install");
        assert_eq!(rendered.toc[0].children[0].children[0].title, "Cargo");
        assert_eq!(rendered.toc[0].children[1].title, "Next");
    }

    #[test]
    fn de_duplicates_duplicate_heading_ids() {
        let rendered = render_html("## Repeat\n\n## Repeat\n\n## Repeat");

        assert!(rendered.html.contains("<h2 id=\"repeat\">Repeat</h2>"));
        assert!(rendered.html.contains("<h2 id=\"repeat-2\">Repeat</h2>"));
        assert!(rendered.html.contains("<h2 id=\"repeat-3\">Repeat</h2>"));
        assert_eq!(rendered.toc[0].id, "repeat");
        assert_eq!(rendered.toc[1].id, "repeat-2");
        assert_eq!(rendered.toc[2].id, "repeat-3");
    }

    #[test]
    fn keeps_inline_heading_markup_and_plain_toc_title() {
        let rendered = render_html("## Intro to `Rustipo`");

        assert!(
            rendered
                .html
                .contains("<h2 id=\"intro-to-rustipo\">Intro to <code>Rustipo</code></h2>")
        );
        assert_eq!(rendered.toc[0].title, "Intro to Rustipo");
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
