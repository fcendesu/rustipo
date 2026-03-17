use super::parser::ParsedShortcode;

pub(super) fn render_shortcode(shortcode: &ParsedShortcode) -> Option<String> {
    match shortcode.name.as_str() {
        "youtube" => render_youtube(shortcode),
        "link" => render_link(shortcode),
        _ => None,
    }
}

fn render_youtube(shortcode: &ParsedShortcode) -> Option<String> {
    let id = shortcode.attrs.get("id")?;
    let id = id.trim();
    if id.is_empty() {
        return None;
    }
    Some(format!(
        "<div class=\"rustipo-shortcode rustipo-youtube\"><iframe src=\"https://www.youtube.com/embed/{}\" title=\"YouTube video\" loading=\"lazy\" allowfullscreen></iframe></div>",
        escape_html(id)
    ))
}

fn render_link(shortcode: &ParsedShortcode) -> Option<String> {
    let href = shortcode.attrs.get("href")?;
    let text = shortcode
        .attrs
        .get("text")
        .cloned()
        .unwrap_or_else(|| href.clone());
    Some(format!(
        "<a class=\"rustipo-shortcode rustipo-link\" href=\"{}\">{}</a>",
        escape_html(href),
        escape_html(&text)
    ))
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}
