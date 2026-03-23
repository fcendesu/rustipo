pub fn base_path(base_url: &str) -> String {
    let without_scheme = base_url
        .split_once("://")
        .map(|(_, rest)| rest)
        .unwrap_or(base_url);

    let path = without_scheme
        .find('/')
        .map(|index| &without_scheme[index..])
        .unwrap_or("/");
    let path = path
        .split(['?', '#'])
        .next()
        .unwrap_or(path)
        .trim_matches('/');

    if path.is_empty() {
        "/".to_string()
    } else {
        format!("/{path}")
    }
}

pub fn public_url_path(base_url: &str, path: &str) -> String {
    if path.is_empty() || is_external_like(path) {
        return path.to_string();
    }

    let normalized = if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    };

    let base = base_path(base_url);
    if base == "/" {
        return normalized;
    }

    if normalized == base || normalized.starts_with(&format!("{base}/")) {
        return normalized;
    }

    format!("{base}{normalized}")
}

fn is_external_like(path: &str) -> bool {
    path.starts_with("http://")
        || path.starts_with("https://")
        || path.starts_with("mailto:")
        || path.starts_with("tel:")
        || path.starts_with("data:")
        || path.starts_with("//")
        || path.starts_with('#')
}

#[cfg(test)]
mod tests {
    use super::{base_path, public_url_path};

    #[test]
    fn extracts_base_path_from_url() {
        assert_eq!(base_path("https://example.com"), "/");
        assert_eq!(base_path("https://example.com/"), "/");
        assert_eq!(base_path("https://example.com/docs/"), "/docs");
        assert_eq!(
            base_path("https://example.com/docs/reference"),
            "/docs/reference"
        );
    }

    #[test]
    fn prefixes_public_paths_with_base_path() {
        assert_eq!(
            public_url_path("https://example.com/docs/", "/guides/"),
            "/docs/guides/"
        );
        assert_eq!(
            public_url_path("https://example.com/docs/", "style.css"),
            "/docs/style.css"
        );
        assert_eq!(
            public_url_path("https://example.com/docs/", "/docs/guides/"),
            "/docs/guides/"
        );
    }

    #[test]
    fn preserves_external_like_paths() {
        assert_eq!(
            public_url_path("https://example.com/docs/", "https://github.com"),
            "https://github.com"
        );
        assert_eq!(
            public_url_path("https://example.com/docs/", "#intro"),
            "#intro"
        );
    }
}
