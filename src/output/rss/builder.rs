use crate::content::pages::Page;

use super::model::FeedItem;

pub(super) fn to_feed_item(page: &Page) -> Option<FeedItem> {
    let date = page.frontmatter.date.as_deref()?;
    let pub_date = normalize_date(date)?;

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

fn normalize_date(date: &str) -> Option<String> {
    if !is_valid_date(date) {
        return None;
    }
    Some(format!("{date}T00:00:00Z"))
}

fn is_valid_date(date: &str) -> bool {
    let parts = date.split('-').collect::<Vec<_>>();
    if parts.len() != 3 {
        return false;
    }

    let year = match parts[0].parse::<u32>() {
        Ok(v) => v,
        Err(_) => return false,
    };
    let month = match parts[1].parse::<u32>() {
        Ok(v) => v,
        Err(_) => return false,
    };
    let day = match parts[2].parse::<u32>() {
        Ok(v) => v,
        Err(_) => return false,
    };

    if !(1..=12).contains(&month) || day == 0 {
        return false;
    }

    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => return false,
    };

    day <= max_day
}

fn is_leap_year(year: u32) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}
