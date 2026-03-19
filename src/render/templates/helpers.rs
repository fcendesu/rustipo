use std::collections::HashMap;

use serde_json::Value;
use tera::{Error as TeraError, Filter, Function, Result as TeraResult, Tera};

use crate::config::SiteConfig;

pub(super) fn register(tera: &mut Tera, config: &SiteConfig) {
    tera.register_filter("slugify", SlugifyFilter);
    tera.register_function(
        "abs_url",
        AbsUrlFunction {
            base_url: config.base_url.clone(),
        },
    );
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
