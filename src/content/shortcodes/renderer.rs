use super::ShortcodeAssets;
use super::parser::ParsedShortcode;

pub(super) struct RenderedShortcode {
    pub html: String,
    pub assets: ShortcodeAssets,
}

pub(super) fn render_shortcode(shortcode: &ParsedShortcode) -> Option<RenderedShortcode> {
    match shortcode.name.as_str() {
        "youtube" => render_youtube(shortcode),
        "link" => render_link(shortcode),
        "iframe" => render_iframe(shortcode),
        "demo" => render_demo(shortcode),
        _ => None,
    }
}

fn render_youtube(shortcode: &ParsedShortcode) -> Option<RenderedShortcode> {
    let id = shortcode.attrs.get("id")?;
    let id = id.trim();
    if id.is_empty() {
        return None;
    }
    Some(RenderedShortcode {
        html: format!(
            "<div class=\"rustipo-shortcode rustipo-youtube\"><iframe src=\"https://www.youtube.com/embed/{}\" title=\"YouTube video\" loading=\"lazy\" allowfullscreen></iframe></div>",
            escape_html(id)
        ),
        assets: ShortcodeAssets::default(),
    })
}

fn render_link(shortcode: &ParsedShortcode) -> Option<RenderedShortcode> {
    let href = shortcode.attrs.get("href")?;
    let text = shortcode
        .attrs
        .get("text")
        .cloned()
        .unwrap_or_else(|| href.clone());
    Some(RenderedShortcode {
        html: format!(
            "<a class=\"rustipo-shortcode rustipo-link\" href=\"{}\">{}</a>",
            escape_html(href),
            escape_html(&text)
        ),
        assets: ShortcodeAssets::default(),
    })
}

fn render_iframe(shortcode: &ParsedShortcode) -> Option<RenderedShortcode> {
    let src = shortcode.attrs.get("src")?.trim();
    if src.is_empty() {
        return None;
    }

    let title = shortcode
        .attrs
        .get("title")
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .unwrap_or("Embedded content");
    let height = shortcode
        .attrs
        .get("height")
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .unwrap_or("420");
    let loading = shortcode
        .attrs
        .get("loading")
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .unwrap_or("lazy");

    Some(RenderedShortcode {
        html: format!(
            "<div class=\"rustipo-shortcode rustipo-iframe\"><iframe src=\"{}\" title=\"{}\" loading=\"{}\" style=\"width: 100%; min-height: {}px; border: 0;\" allowfullscreen></iframe></div>",
            escape_html_attr(src),
            escape_html_attr(title),
            escape_html_attr(loading),
            escape_html_attr(height),
        ),
        assets: ShortcodeAssets::default(),
    })
}

fn render_demo(shortcode: &ParsedShortcode) -> Option<RenderedShortcode> {
    let id = shortcode.attrs.get("id")?.trim();
    if id.is_empty() {
        return None;
    }

    let mut assets = ShortcodeAssets::default();
    if let Some(style) = shortcode
        .attrs
        .get("style")
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
    {
        assets.stylesheets.push(style.to_string());
    }
    if let Some(script) = shortcode
        .attrs
        .get("script")
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
    {
        assets.scripts.push(script.to_string());
    }

    let title = shortcode
        .attrs
        .get("title")
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .unwrap_or(id);

    Some(RenderedShortcode {
        html: format!(
            "<div class=\"rustipo-shortcode rustipo-demo\" data-rustipo-demo=\"{}\"><div class=\"rustipo-demo-fallback\">Interactive demo: {}</div></div>",
            escape_html_attr(id),
            escape_html(title),
        ),
        assets,
    })
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
