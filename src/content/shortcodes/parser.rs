use std::collections::BTreeMap;

pub(super) struct ParsedShortcode {
    pub name: String,
    pub attrs: BTreeMap<String, String>,
}

pub(super) fn parse_shortcode(input: &str) -> Option<ParsedShortcode> {
    let (name, attrs_raw) = split_name_and_attrs(input)?;
    let attrs = parse_attrs(attrs_raw)?;
    Some(ParsedShortcode {
        name: name.to_string(),
        attrs,
    })
}

fn split_name_and_attrs(input: &str) -> Option<(&str, &str)> {
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

fn parse_attrs(input: &str) -> Option<BTreeMap<String, String>> {
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
