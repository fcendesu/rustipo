use crate::content::pages::Page;

use super::model::FeedItem;

pub(super) fn to_feed_item(page: &Page) -> Option<FeedItem> {
    let date = page.frontmatter.date.as_ref()?;
    let pub_date = format!("{}T00:00:00Z", date.as_str());

    let title = page
        .frontmatter
        .title
        .clone()
        .unwrap_or_else(|| page.slug.clone());

    let description = page
        .frontmatter
        .summary
        .clone()
        .unwrap_or_else(|| title.clone());

    Some(FeedItem {
        title,
        link: page.route.clone(),
        description,
        pub_date,
    })
}
