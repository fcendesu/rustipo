mod parser;
mod renderer;

use parser::parse_shortcode;
use renderer::{RenderedShortcode, render_shortcode};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ShortcodeAssets {
    pub stylesheets: Vec<String>,
    pub scripts: Vec<String>,
}

impl ShortcodeAssets {
    fn merge(&mut self, other: ShortcodeAssets) {
        for stylesheet in other.stylesheets {
            push_unique(&mut self.stylesheets, stylesheet);
        }

        for script in other.scripts {
            push_unique(&mut self.scripts, script);
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PreprocessedMarkdown {
    pub markdown: String,
    pub assets: ShortcodeAssets,
}

pub fn preprocess(markdown: &str) -> PreprocessedMarkdown {
    let mut output = String::with_capacity(markdown.len());
    let mut assets = ShortcodeAssets::default();
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

        let rendered = replace_shortcodes_in_text(line);
        output.push_str(&rendered.markdown);
        assets.merge(rendered.assets);
    }

    PreprocessedMarkdown {
        markdown: output,
        assets,
    }
}

fn replace_shortcodes_in_text(input: &str) -> PreprocessedMarkdown {
    let mut output = String::with_capacity(input.len());
    let mut assets = ShortcodeAssets::default();
    let mut cursor = 0;

    while let Some(start_rel) = input[cursor..].find("{{<") {
        let start = cursor + start_rel;
        output.push_str(&input[cursor..start]);

        let Some(end_rel) = input[start + 3..].find(">}}") else {
            output.push_str(&input[start..]);
            return PreprocessedMarkdown {
                markdown: output,
                assets,
            };
        };
        let end = start + 3 + end_rel + 3;
        let raw = &input[start..end];
        let inner = input[start + 3..start + 3 + end_rel].trim();

        if let Some(parsed) = parse_shortcode(inner) {
            if let Some(RenderedShortcode {
                html,
                assets: shortcode_assets,
            }) = render_shortcode(&parsed)
            {
                output.push_str(&html);
                assets.merge(shortcode_assets);
            } else {
                output.push_str(raw);
            }
        } else {
            output.push_str(raw);
        }

        cursor = end;
    }

    output.push_str(&input[cursor..]);
    PreprocessedMarkdown {
        markdown: output,
        assets,
    }
}

fn push_unique(values: &mut Vec<String>, candidate: String) {
    if !values.iter().any(|existing| existing == &candidate) {
        values.push(candidate);
    }
}

#[cfg(test)]
mod tests {
    use super::preprocess;

    #[test]
    fn renders_youtube_shortcode() {
        let rendered = preprocess("{{< youtube id=\"dQw4w9WgXcQ\" >}}");
        assert!(rendered.markdown.contains("youtube.com/embed/dQw4w9WgXcQ"));
        assert!(rendered.markdown.contains("rustipo-youtube"));
    }

    #[test]
    fn renders_link_shortcode() {
        let rendered = preprocess("{{< link href=\"https://example.com\" text=\"Visit\" >}}");
        assert!(rendered.markdown.contains("href=\"https://example.com\""));
        assert!(rendered.markdown.contains(">Visit<"));
    }

    #[test]
    fn leaves_unknown_shortcode_as_text() {
        let rendered = preprocess("{{< unknown foo=\"bar\" >}}");
        assert_eq!(rendered.markdown, "{{< unknown foo=\"bar\" >}}");
    }

    #[test]
    fn does_not_render_shortcode_inside_code_fence() {
        let rendered = preprocess("```\n{{< youtube id=\"dQw4w9WgXcQ\" >}}\n```");
        assert!(!rendered.markdown.contains("youtube.com/embed/"));
    }

    #[test]
    fn renders_iframe_shortcode() {
        let rendered = preprocess(
            "{{< iframe src=\"https://example.com/demo\" title=\"Example demo\" height=\"420\" >}}",
        );

        assert!(rendered.markdown.contains("rustipo-iframe"));
        assert!(
            rendered
                .markdown
                .contains("src=\"https://example.com/demo\"")
        );
        assert!(rendered.markdown.contains("title=\"Example demo\""));
        assert!(rendered.markdown.contains("height: 420px"));
    }

    #[test]
    fn collects_deduplicated_demo_assets() {
        let rendered = preprocess(
            "{{< demo id=\"counter\" script=\"/demos/counter.js\" style=\"/demos/counter.css\" >}}\n\
             {{< demo id=\"counter\" script=\"/demos/counter.js\" style=\"/demos/counter.css\" >}}",
        );

        assert!(rendered.markdown.contains("data-rustipo-demo=\"counter\""));
        assert_eq!(rendered.assets.scripts, vec!["/demos/counter.js"]);
        assert_eq!(rendered.assets.stylesheets, vec!["/demos/counter.css"]);
    }

    #[test]
    fn renders_demo_shortcode_after_fenced_shortcode_examples() {
        let rendered = preprocess(
            "```md\n{{< demo id=\"counter-demo\" script=\"/demos/counter-demo.js\" style=\"/demos/counter-demo.css\" title=\"Counter demo\" >}}\n```\n\n{{< demo id=\"counter-demo\" script=\"/demos/counter-demo.js\" style=\"/demos/counter-demo.css\" title=\"Counter demo\" >}}\n",
        );

        assert!(rendered.markdown.contains("{{< demo id=\"counter-demo\""));
        assert!(
            rendered
                .markdown
                .contains("data-rustipo-demo=\"counter-demo\"")
        );
    }
}
