use std::collections::HashMap;

use chrono::NaiveDate;
use serde_json::Value;
use tera::{Error as TeraError, Filter, Function, Result as TeraResult, Tera};

use crate::config::SiteConfig;

pub(super) fn register(tera: &mut Tera, config: &SiteConfig) {
    tera.register_filter("slugify", SlugifyFilter);
    tera.register_filter("format_date", FormatDateFilter);
    tera.register_function(
        "abs_url",
        AbsUrlFunction {
            base_url: config.base_url.clone(),
        },
    );
    tera.register_function("asset_url", AssetUrlFunction);
    tera.register_function("tag_url", TagUrlFunction);
}

struct SlugifyFilter;

impl Filter for SlugifyFilter {
    fn filter(&self, value: &Value, _: &HashMap<String, Value>) -> TeraResult<Value> {
        let input = value
            .as_str()
            .ok_or_else(|| TeraError::msg("slugify filter expects a string value"))?;
        Ok(Value::String(slugify(input)))
    }

    fn is_safe(&self) -> bool {
        true
    }
}

struct AbsUrlFunction {
    base_url: String,
}

struct FormatDateFilter;
struct AssetUrlFunction;
struct TagUrlFunction;

impl Function for AbsUrlFunction {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let path = args
            .get("path")
            .and_then(Value::as_str)
            .ok_or_else(|| TeraError::msg("abs_url requires a string 'path' argument"))?;

        if path.starts_with("http://") || path.starts_with("https://") {
            return Ok(Value::String(path.to_string()));
        }

        let base = self.base_url.trim_end_matches('/');
        let normalized = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{path}")
        };

        Ok(Value::String(format!("{base}{normalized}")))
    }
}

impl Filter for FormatDateFilter {
    fn filter(&self, value: &Value, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let raw = value
            .as_str()
            .ok_or_else(|| TeraError::msg("format_date filter expects a string value"))?;
        let format = args
            .get("format")
            .and_then(Value::as_str)
            .unwrap_or("%Y-%m-%d");

        let date = NaiveDate::parse_from_str(raw, "%Y-%m-%d")
            .map_err(|_| TeraError::msg("format_date filter expects a YYYY-MM-DD string"))?;

        Ok(Value::String(date.format(format).to_string()))
    }

    fn is_safe(&self) -> bool {
        true
    }
}

impl Function for AssetUrlFunction {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let path = args
            .get("path")
            .and_then(Value::as_str)
            .ok_or_else(|| TeraError::msg("asset_url requires a string 'path' argument"))?;

        Ok(Value::String(normalize_url_path(path)))
    }
}

impl Function for TagUrlFunction {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let name = args
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| TeraError::msg("tag_url requires a string 'name' argument"))?;
        let slug = slugify(name);
        if slug.is_empty() {
            return Err(TeraError::msg(
                "tag_url requires at least one ASCII letter or digit",
            ));
        }

        Ok(Value::String(format!("/tags/{slug}/")))
    }
}

fn normalize_url_path(path: &str) -> String {
    if path.starts_with("http://") || path.starts_with("https://") {
        return path.to_string();
    }

    if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    }
}

fn slugify(input: &str) -> String {
    let mut slug = String::with_capacity(input.len());
    let mut previous_dash = false;

    for ch in input.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            previous_dash = false;
        } else if !previous_dash {
            slug.push('-');
            previous_dash = true;
        }
    }

    slug.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::Value;
    use tera::{Filter, Function};

    use super::{AssetUrlFunction, FormatDateFilter, TagUrlFunction};

    #[test]
    fn formats_iso_dates() {
        let filter = FormatDateFilter;
        let mut args = HashMap::new();
        args.insert("format".to_string(), Value::String("%B %d, %Y".to_string()));

        let value = filter
            .filter(&Value::String("2026-03-19".to_string()), &args)
            .expect("date should format");

        assert_eq!(value, Value::String("March 19, 2026".to_string()));
    }

    #[test]
    fn normalizes_asset_urls() {
        let function = AssetUrlFunction;
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            Value::String("img/logo.svg".to_string()),
        );

        let value = function.call(&args).expect("asset url should render");
        assert_eq!(value, Value::String("/img/logo.svg".to_string()));
    }

    #[test]
    fn builds_tag_urls() {
        let function = TagUrlFunction;
        let mut args = HashMap::new();
        args.insert("name".to_string(), Value::String("Site Gen".to_string()));

        let value = function.call(&args).expect("tag url should render");
        assert_eq!(value, Value::String("/tags/site-gen/".to_string()));
    }
}
