use pulldown_cmark::{Options, Parser, html};

pub fn render_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(markdown, options);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
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
}
