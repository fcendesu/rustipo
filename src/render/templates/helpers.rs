use std::collections::HashMap;
use std::sync::Arc;

use chrono::NaiveDate;
use serde_json::Value;
use tera::{Error as TeraError, Filter, Function, Result as TeraResult, Tera};

use crate::config::SiteConfig;
use crate::images::{ImageProcessor, OutputFormat, ResizeOperation, ResizeRequest};
use crate::taxonomy::{TAGS_TAXONOMY, slugify_term, taxonomy_route, taxonomy_term_route};

pub(super) fn register(
    tera: &mut Tera,
    config: &SiteConfig,
    image_processor: Option<Arc<ImageProcessor>>,
) {
    tera.register_filter("slugify", SlugifyFilter);
    tera.register_filter("format_date", FormatDateFilter);
    tera.register_function(
        "abs_url",
        AbsUrlFunction {
            base_url: config.base_url.clone(),
        },
    );
    tera.register_function(
        "asset_url",
        AssetUrlFunction {
            base_url: config.base_url.clone(),
        },
    );
    tera.register_function(
        "tag_url",
        TagUrlFunction {
            base_url: config.base_url.clone(),
        },
    );
    tera.register_function(
        "taxonomy_url",
        TaxonomyUrlFunction {
            base_url: config.base_url.clone(),
        },
    );
    if let Some(processor) = image_processor {
        tera.register_function("resize_image", ResizeImageFunction { processor });
    }
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
struct AssetUrlFunction {
    base_url: String,
}
struct TagUrlFunction {
    base_url: String,
}
struct TaxonomyUrlFunction {
    base_url: String,
}
struct ResizeImageFunction {
    processor: Arc<ImageProcessor>,
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

        Ok(Value::String(crate::url::public_url_path(
            &self.base_url,
            path,
        )))
    }
}

impl Function for TagUrlFunction {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let term = args
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(|| TeraError::msg("tag_url requires a string 'name' argument"))?;
        taxonomy_url_value(&self.base_url, TAGS_TAXONOMY, term, "tag_url")
    }
}

impl Function for TaxonomyUrlFunction {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let taxonomy = args
            .get("taxonomy")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                TeraError::msg("taxonomy_url requires a non-empty string 'taxonomy' argument")
            })?;
        let term = args
            .get("term")
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                TeraError::msg("taxonomy_url requires a non-empty string 'term' argument")
            })?;

        taxonomy_url_value(&self.base_url, taxonomy, term, "taxonomy_url")
    }
}

impl Function for ResizeImageFunction {
    fn call(&self, args: &HashMap<String, Value>) -> TeraResult<Value> {
        let path = required_string_arg(args, "path")?;
        let op = parse_optional_resize_op(args)?.unwrap_or(ResizeOperation::Fill);
        let format = parse_optional_output_format(args)?.unwrap_or(OutputFormat::Auto);
        let request = ResizeRequest {
            path,
            width: optional_u32_arg(args, "width")?,
            height: optional_u32_arg(args, "height")?,
            op,
            format,
            quality: optional_u8_arg(args, "quality")?,
        };

        let processed = self
            .processor
            .resize(&request)
            .map_err(|error| TeraError::msg(error.to_string()))?;
        serde_json::to_value(processed).map_err(|error| {
            TeraError::msg(format!("resize_image failed to serialize result: {error}"))
        })
    }
}

fn required_string_arg(args: &HashMap<String, Value>, name: &str) -> TeraResult<String> {
    args.get(name)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .ok_or_else(|| {
            TeraError::msg(format!(
                "resize_image requires a non-empty string '{name}' argument"
            ))
        })
}

fn optional_string_arg(args: &HashMap<String, Value>, name: &str) -> Option<String> {
    args.get(name)
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn optional_u32_arg(args: &HashMap<String, Value>, name: &str) -> TeraResult<Option<u32>> {
    match args.get(name) {
        None => Ok(None),
        Some(value) => {
            let Some(raw) = value.as_u64() else {
                return Err(TeraError::msg(format!(
                    "resize_image '{name}' must be a positive integer"
                )));
            };
            let converted = u32::try_from(raw).map_err(|_| {
                TeraError::msg(format!(
                    "resize_image '{name}' must fit into a 32-bit integer"
                ))
            })?;
            if converted == 0 {
                return Err(TeraError::msg(format!(
                    "resize_image '{name}' must be greater than zero"
                )));
            }
            Ok(Some(converted))
        }
    }
}

fn optional_u8_arg(args: &HashMap<String, Value>, name: &str) -> TeraResult<Option<u8>> {
    match args.get(name) {
        None => Ok(None),
        Some(value) => {
            let Some(raw) = value.as_u64() else {
                return Err(TeraError::msg(format!(
                    "resize_image '{name}' must be a positive integer"
                )));
            };
            let converted = u8::try_from(raw).map_err(|_| {
                TeraError::msg(format!(
                    "resize_image '{name}' must fit into an 8-bit integer"
                ))
            })?;
            Ok(Some(converted))
        }
    }
}

fn parse_optional_resize_op(args: &HashMap<String, Value>) -> TeraResult<Option<ResizeOperation>> {
    match optional_string_arg(args, "op") {
        None => Ok(None),
        Some(value) => ResizeOperation::parse(&value).map(Some).ok_or_else(|| {
            TeraError::msg("resize_image 'op' must be one of: fit_width, fit_height, fit, fill")
        }),
    }
}

fn parse_optional_output_format(args: &HashMap<String, Value>) -> TeraResult<Option<OutputFormat>> {
    match optional_string_arg(args, "format") {
        None => Ok(None),
        Some(value) => OutputFormat::parse(&value).map(Some).ok_or_else(|| {
            TeraError::msg("resize_image 'format' must be one of: auto, jpg, png, webp")
        }),
    }
}

fn taxonomy_url_value(
    base_url: &str,
    taxonomy: &str,
    term: &str,
    helper_name: &str,
) -> TeraResult<Value> {
    if taxonomy_route(taxonomy).is_none() {
        return Err(TeraError::msg(format!(
            "{helper_name} does not support taxonomy '{taxonomy}'; supported taxonomies: {TAGS_TAXONOMY}"
        )));
    }

    let slug = slugify_term(term);
    if slug.is_empty() {
        return Err(TeraError::msg(format!(
            "{helper_name} requires at least one ASCII letter or digit in 'term'"
        )));
    }

    Ok(Value::String(crate::url::public_url_path(
        base_url,
        &taxonomy_term_route(taxonomy, &slug)
            .expect("taxonomy term route should exist for supported taxonomy"),
    )))
}

fn slugify(input: &str) -> String {
    slugify_term(input)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use serde_json::Value;
    use tera::{Filter, Function};

    use super::{AssetUrlFunction, FormatDateFilter, TagUrlFunction, TaxonomyUrlFunction};

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
        let function = AssetUrlFunction {
            base_url: "https://example.com/docs/".to_string(),
        };
        let mut args = HashMap::new();
        args.insert(
            "path".to_string(),
            Value::String("img/logo.svg".to_string()),
        );

        let value = function.call(&args).expect("asset url should render");
        assert_eq!(value, Value::String("/docs/img/logo.svg".to_string()));
    }

    #[test]
    fn builds_tag_urls() {
        let function = TagUrlFunction {
            base_url: "https://example.com/docs/".to_string(),
        };
        let mut args = HashMap::new();
        args.insert("name".to_string(), Value::String("Site Gen".to_string()));

        let value = function.call(&args).expect("tag url should render");
        assert_eq!(value, Value::String("/docs/tags/site-gen/".to_string()));
    }

    #[test]
    fn builds_taxonomy_urls() {
        let function = TaxonomyUrlFunction {
            base_url: "https://example.com/docs/".to_string(),
        };
        let mut args = HashMap::new();
        args.insert("taxonomy".to_string(), Value::String("tags".to_string()));
        args.insert("term".to_string(), Value::String("Rust Tips".to_string()));

        let value = function.call(&args).expect("taxonomy url should render");
        assert_eq!(value, Value::String("/docs/tags/rust-tips/".to_string()));
    }

    #[test]
    fn rejects_unknown_taxonomy_urls() {
        let function = TaxonomyUrlFunction {
            base_url: "https://example.com/docs/".to_string(),
        };
        let mut args = HashMap::new();
        args.insert(
            "taxonomy".to_string(),
            Value::String("categories".to_string()),
        );
        args.insert("term".to_string(), Value::String("Rust Tips".to_string()));

        let error = function
            .call(&args)
            .expect_err("taxonomy should be rejected");
        assert!(error.to_string().contains("supported taxonomies: tags"));
    }
}
