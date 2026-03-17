mod parser;
mod renderer;

use parser::parse_shortcode;
use renderer::render_shortcode;

pub fn preprocess(markdown: &str) -> String {
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

        if let Some(parsed) = parse_shortcode(inner) {
            if let Some(rendered) = render_shortcode(&parsed) {
                output.push_str(&rendered);
            } else {
                output.push_str(raw);
            }
        } else {
            output.push_str(raw);
        }

        cursor = end;
    }

    output.push_str(&input[cursor..]);
    output
}

#[cfg(test)]
mod tests {
    use super::preprocess;

    #[test]
    fn renders_youtube_shortcode() {
        let html = preprocess("{{< youtube id=\"dQw4w9WgXcQ\" >}}");
        assert!(html.contains("youtube.com/embed/dQw4w9WgXcQ"));
        assert!(html.contains("rustipo-youtube"));
    }

    #[test]
    fn renders_link_shortcode() {
        let html = preprocess("{{< link href=\"https://example.com\" text=\"Visit\" >}}");
        assert!(html.contains("href=\"https://example.com\""));
        assert!(html.contains(">Visit<"));
    }

    #[test]
    fn leaves_unknown_shortcode_as_text() {
        let html = preprocess("{{< unknown foo=\"bar\" >}}");
        assert_eq!(html, "{{< unknown foo=\"bar\" >}}");
    }

    #[test]
    fn does_not_render_shortcode_inside_code_fence() {
        let html = preprocess("```\n{{< youtube id=\"dQw4w9WgXcQ\" >}}\n```");
        assert!(!html.contains("youtube.com/embed/"));
    }
}
