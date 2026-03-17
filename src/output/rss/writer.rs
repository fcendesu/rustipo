use crate::config::SiteConfig;

use super::model::FeedItem;

pub(super) fn render_feed(config: &SiteConfig, entries: &[FeedItem]) -> String {
    let mut xml = String::new();

    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<rss version=\"2.0\">\n");
    xml.push_str("  <channel>\n");
    xml.push_str(&format!(
        "    <title>{}</title>\n",
        escape_xml(&config.title)
    ));
    xml.push_str(&format!(
        "    <description>{}</description>\n",
        escape_xml(&config.description)
    ));
    xml.push_str(&format!(
        "    <link>{}</link>\n",
        escape_xml(&config.base_url)
    ));

    for entry in entries {
        let absolute_link = format!("{}{}", config.base_url.trim_end_matches('/'), entry.link);
        xml.push_str("    <item>\n");
        xml.push_str(&format!(
            "      <title>{}</title>\n",
            escape_xml(&entry.title)
        ));
        xml.push_str(&format!(
            "      <link>{}</link>\n",
            escape_xml(&absolute_link)
        ));
        xml.push_str(&format!(
            "      <description>{}</description>\n",
            escape_xml(&entry.description)
        ));
        xml.push_str(&format!("      <pubDate>{}</pubDate>\n", entry.pub_date));
        xml.push_str("    </item>\n");
    }

    xml.push_str("  </channel>\n");
    xml.push_str("</rss>\n");
    xml
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
