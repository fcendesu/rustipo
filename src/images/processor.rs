use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageFormat, ImageReader};
use serde::Serialize;

use super::PROCESSED_IMAGES_DIR;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ProcessedImage {
    pub url: String,
    pub static_path: String,
    pub width: u32,
    pub height: u32,
    pub orig_width: u32,
    pub orig_height: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResizeOperation {
    FitWidth,
    FitHeight,
    Fit,
    Fill,
}

impl ResizeOperation {
    pub fn parse(input: &str) -> Option<Self> {
        match input {
            "fit_width" => Some(Self::FitWidth),
            "fit_height" => Some(Self::FitHeight),
            "fit" => Some(Self::Fit),
            "fill" => Some(Self::Fill),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Auto,
    Jpg,
    Png,
    Webp,
}

impl OutputFormat {
    pub fn parse(input: &str) -> Option<Self> {
        match input {
            "auto" => Some(Self::Auto),
            "jpg" | "jpeg" => Some(Self::Jpg),
            "png" => Some(Self::Png),
            "webp" => Some(Self::Webp),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResizeRequest {
    pub path: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub op: ResizeOperation,
    pub format: OutputFormat,
    pub quality: Option<u8>,
}

#[derive(Debug, Clone)]
pub struct ImageProcessor {
    base_url: String,
    project_root: PathBuf,
    output_root: PathBuf,
    theme_static_dirs: Vec<PathBuf>,
}

impl ImageProcessor {
    pub fn new(
        project_root: impl AsRef<Path>,
        output_root: impl AsRef<Path>,
        base_url: &str,
        theme_static_dirs: &[PathBuf],
    ) -> Self {
        Self {
            base_url: base_url.to_string(),
            project_root: project_root.as_ref().to_path_buf(),
            output_root: output_root.as_ref().to_path_buf(),
            theme_static_dirs: theme_static_dirs.to_vec(),
        }
    }

    pub fn resize(&self, request: &ResizeRequest) -> Result<ProcessedImage> {
        validate_request(request)?;

        let source_path = self.resolve_source_path(&request.path)?;
        let source_bytes = fs::read(&source_path)
            .with_context(|| format!("failed to read source image: {}", source_path.display()))?;
        let source_format = ImageReader::new(Cursor::new(&source_bytes))
            .with_guessed_format()
            .context("failed to detect source image format")?
            .format();
        let image = ImageReader::new(Cursor::new(&source_bytes))
            .with_guessed_format()
            .context("failed to detect source image format")?
            .decode()
            .with_context(|| format!("failed to decode image: {}", source_path.display()))?;

        let (orig_width, orig_height) = image.dimensions();
        let resized = apply_resize(&image, request)?;
        let (width, height) = resized.dimensions();
        let resolved_format = resolve_output_format(request.format, source_format);
        let file_name = build_output_file_name(&source_bytes, request, resolved_format);
        let static_path = format!("{PROCESSED_IMAGES_DIR}/{file_name}");
        let output_path = self.output_root.join(&static_path);

        if !output_path.is_file() {
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent).with_context(|| {
                    format!(
                        "failed to create processed image output path: {}",
                        parent.display()
                    )
                })?;
            }

            let encoded = encode_image(&resized, resolved_format, request.quality)?;
            fs::write(&output_path, encoded).with_context(|| {
                format!(
                    "failed to write processed image output: {}",
                    output_path.display()
                )
            })?;
        }

        Ok(ProcessedImage {
            url: crate::url::public_url_path(&self.base_url, &format!("/{static_path}")),
            static_path,
            width,
            height,
            orig_width,
            orig_height,
        })
    }

    fn resolve_source_path(&self, path: &str) -> Result<PathBuf> {
        let trimmed = path.trim();
        if trimmed.is_empty() {
            bail!("resize_image requires a non-empty 'path' argument");
        }

        let candidate = Path::new(trimmed);
        if candidate.is_absolute() && candidate.is_file() {
            return Ok(candidate.to_path_buf());
        }

        let normalized = trimmed.trim_start_matches('/');
        let mut roots = vec![
            self.project_root.clone(),
            self.project_root.join("static"),
            self.project_root.join("content"),
            self.project_root.join("public"),
        ];
        for dir in self.theme_static_dirs.iter().rev() {
            roots.push(dir.clone());
        }

        for root in roots {
            let resolved = root.join(normalized);
            if resolved.is_file() {
                return Ok(resolved);
            }
        }

        bail!(
            "resize_image could not find source image for path '{}': searched project root, static/, content/, public/, and theme static directories",
            path
        )
    }
}

fn validate_request(request: &ResizeRequest) -> Result<()> {
    match request.op {
        ResizeOperation::FitWidth => {
            if request.width.is_none() {
                bail!("resize_image op='fit_width' requires a positive 'width' argument");
            }
        }
        ResizeOperation::FitHeight => {
            if request.height.is_none() {
                bail!("resize_image op='fit_height' requires a positive 'height' argument");
            }
        }
        ResizeOperation::Fit | ResizeOperation::Fill => {
            if request.width.is_none() || request.height.is_none() {
                bail!(
                    "resize_image op='{}' requires both positive 'width' and 'height' arguments",
                    op_name(request.op)
                );
            }
        }
    }

    if let Some(quality) = request.quality
        && !(1..=100).contains(&quality)
    {
        bail!("resize_image quality must be between 1 and 100");
    }

    Ok(())
}

fn apply_resize(image: &DynamicImage, request: &ResizeRequest) -> Result<DynamicImage> {
    let (orig_width, orig_height) = image.dimensions();

    Ok(match request.op {
        ResizeOperation::FitWidth => {
            let width = request.width.expect("validated width");
            let height = scaled_dimension(orig_height, orig_width, width);
            image.resize_exact(width, height, FilterType::Lanczos3)
        }
        ResizeOperation::FitHeight => {
            let height = request.height.expect("validated height");
            let width = scaled_dimension(orig_width, orig_height, height);
            image.resize_exact(width, height, FilterType::Lanczos3)
        }
        ResizeOperation::Fit => {
            let width = request.width.expect("validated width");
            let height = request.height.expect("validated height");
            let width_ratio = width as f64 / orig_width as f64;
            let height_ratio = height as f64 / orig_height as f64;
            let scale = width_ratio.min(height_ratio).min(1.0);
            if (scale - 1.0).abs() < f64::EPSILON {
                image.clone()
            } else {
                let new_width = ((orig_width as f64 * scale).round() as u32).max(1);
                let new_height = ((orig_height as f64 * scale).round() as u32).max(1);
                image.resize_exact(new_width, new_height, FilterType::Lanczos3)
            }
        }
        ResizeOperation::Fill => {
            let width = request.width.expect("validated width");
            let height = request.height.expect("validated height");
            let target_ratio = width as f64 / height as f64;
            let source_ratio = orig_width as f64 / orig_height as f64;

            let (crop_x, crop_y, crop_width, crop_height) = if source_ratio > target_ratio {
                let crop_width = ((orig_height as f64 * target_ratio).round() as u32)
                    .max(1)
                    .min(orig_width);
                ((orig_width - crop_width) / 2, 0, crop_width, orig_height)
            } else {
                let crop_height = ((orig_width as f64 / target_ratio).round() as u32)
                    .max(1)
                    .min(orig_height);
                (0, (orig_height - crop_height) / 2, orig_width, crop_height)
            };

            image
                .crop_imm(crop_x, crop_y, crop_width, crop_height)
                .resize_exact(width, height, FilterType::Lanczos3)
        }
    })
}

fn scaled_dimension(target_side: u32, original_side: u32, requested_side: u32) -> u32 {
    (((target_side as f64 * requested_side as f64) / original_side as f64).round() as u32).max(1)
}

fn resolve_output_format(
    requested: OutputFormat,
    source_format: Option<ImageFormat>,
) -> OutputFormat {
    match requested {
        OutputFormat::Auto => match source_format {
            Some(ImageFormat::Png) => OutputFormat::Png,
            Some(ImageFormat::Gif)
            | Some(ImageFormat::Bmp)
            | Some(ImageFormat::Tiff)
            | Some(ImageFormat::Pnm)
            | Some(ImageFormat::Tga)
            | Some(ImageFormat::Dds)
            | Some(ImageFormat::Farbfeld)
            | Some(ImageFormat::OpenExr)
            | Some(ImageFormat::Qoi)
            | Some(ImageFormat::Ico) => OutputFormat::Png,
            _ => OutputFormat::Jpg,
        },
        other => other,
    }
}

fn build_output_file_name(
    source_bytes: &[u8],
    request: &ResizeRequest,
    format: OutputFormat,
) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    hash_bytes(&mut hash, source_bytes);
    hash_bytes(&mut hash, request.path.as_bytes());
    hash_bytes(&mut hash, &[0]);
    hash_bytes(&mut hash, op_name(request.op).as_bytes());
    hash_bytes(&mut hash, &[0]);
    hash_bytes(
        &mut hash,
        request.width.unwrap_or_default().to_string().as_bytes(),
    );
    hash_bytes(&mut hash, &[0]);
    hash_bytes(
        &mut hash,
        request.height.unwrap_or_default().to_string().as_bytes(),
    );
    hash_bytes(&mut hash, &[0]);
    hash_bytes(
        &mut hash,
        request.quality.unwrap_or_default().to_string().as_bytes(),
    );
    hash_bytes(&mut hash, &[0]);
    hash_bytes(&mut hash, format_extension(format).as_bytes());

    format!("{hash:016x}.{}", format_extension(format))
}

fn hash_bytes(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        *hash ^= u64::from(*byte);
        *hash = hash.wrapping_mul(0x100000001b3);
    }
}

fn encode_image(
    image: &DynamicImage,
    format: OutputFormat,
    quality: Option<u8>,
) -> Result<Vec<u8>> {
    let mut bytes = Vec::new();

    match format {
        OutputFormat::Jpg => {
            let mut encoder = JpegEncoder::new_with_quality(&mut bytes, quality.unwrap_or(75));
            encoder
                .encode_image(image)
                .context("failed to encode JPEG derivative")?;
        }
        OutputFormat::Png => {
            let mut cursor = Cursor::new(Vec::new());
            image
                .write_to(&mut cursor, ImageFormat::Png)
                .context("failed to encode PNG derivative")?;
            bytes = cursor.into_inner();
        }
        OutputFormat::Webp => {
            let mut cursor = Cursor::new(Vec::new());
            image
                .write_to(&mut cursor, ImageFormat::WebP)
                .context("failed to encode WebP derivative")?;
            bytes = cursor.into_inner();
        }
        OutputFormat::Auto => unreachable!("auto format should resolve before encoding"),
    }

    Ok(bytes)
}

fn format_extension(format: OutputFormat) -> &'static str {
    match format {
        OutputFormat::Auto => "auto",
        OutputFormat::Jpg => "jpg",
        OutputFormat::Png => "png",
        OutputFormat::Webp => "webp",
    }
}

fn op_name(op: ResizeOperation) -> &'static str {
    match op {
        ResizeOperation::FitWidth => "fit_width",
        ResizeOperation::FitHeight => "fit_height",
        ResizeOperation::Fit => "fit",
        ResizeOperation::Fill => "fill",
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use image::{ImageBuffer, Rgba};
    use tempfile::tempdir;

    use super::{
        ImageProcessor, OutputFormat, ResizeOperation, ResizeRequest, build_output_file_name,
    };

    fn write_sample_png(path: &Path, width: u32, height: u32) {
        let image = ImageBuffer::from_fn(width, height, |x, y| {
            let r = (x * 20) as u8;
            let g = (y * 30) as u8;
            Rgba([r, g, 180, 255])
        });
        image.save(path).expect("sample png should write");
    }

    use std::path::Path;

    #[test]
    fn resolves_project_static_images_and_writes_processed_output() {
        let dir = tempdir().expect("tempdir should be created");
        fs::create_dir_all(dir.path().join("static/images")).expect("static dir should exist");
        write_sample_png(&dir.path().join("static/images/cover.png"), 16, 8);

        let processor = ImageProcessor::new(
            dir.path(),
            dir.path().join("dist"),
            "https://example.com/docs/",
            &[],
        );
        let processed = processor
            .resize(&ResizeRequest {
                path: "/images/cover.png".to_string(),
                width: Some(8),
                height: Some(8),
                op: ResizeOperation::Fit,
                format: OutputFormat::Png,
                quality: None,
            })
            .expect("resize should succeed");

        assert_eq!(processed.width, 8);
        assert_eq!(processed.height, 4);
        assert_eq!(processed.orig_width, 16);
        assert_eq!(processed.orig_height, 8);
        assert!(processed.url.starts_with("/docs/processed-images/"));
        assert!(
            dir.path()
                .join("dist")
                .join(&processed.static_path)
                .is_file()
        );
    }

    #[test]
    fn fit_width_upscales_based_on_requested_width() {
        let dir = tempdir().expect("tempdir should be created");
        fs::create_dir_all(dir.path().join("static")).expect("static dir should exist");
        write_sample_png(&dir.path().join("static/sample.png"), 10, 5);

        let processor = ImageProcessor::new(
            dir.path(),
            dir.path().join("dist"),
            "https://example.com",
            &[],
        );
        let processed = processor
            .resize(&ResizeRequest {
                path: "sample.png".to_string(),
                width: Some(20),
                height: None,
                op: ResizeOperation::FitWidth,
                format: OutputFormat::Png,
                quality: None,
            })
            .expect("resize should succeed");

        assert_eq!(processed.width, 20);
        assert_eq!(processed.height, 10);
    }

    #[test]
    fn fill_crops_to_requested_aspect_ratio() {
        let dir = tempdir().expect("tempdir should be created");
        fs::create_dir_all(dir.path().join("static")).expect("static dir should exist");
        write_sample_png(&dir.path().join("static/sample.png"), 12, 8);

        let processor = ImageProcessor::new(
            dir.path(),
            dir.path().join("dist"),
            "https://example.com",
            &[],
        );
        let processed = processor
            .resize(&ResizeRequest {
                path: "sample.png".to_string(),
                width: Some(6),
                height: Some(6),
                op: ResizeOperation::Fill,
                format: OutputFormat::Png,
                quality: None,
            })
            .expect("resize should succeed");

        assert_eq!(processed.width, 6);
        assert_eq!(processed.height, 6);
    }

    #[test]
    fn prefers_child_theme_static_dir_when_source_exists_in_multiple_layers() {
        let dir = tempdir().expect("tempdir should be created");
        let parent = dir.path().join("themes/base/static/images");
        let child = dir.path().join("themes/child/static/images");
        fs::create_dir_all(&parent).expect("parent dir should exist");
        fs::create_dir_all(&child).expect("child dir should exist");
        write_sample_png(&parent.join("shared.png"), 8, 8);
        write_sample_png(&child.join("shared.png"), 12, 6);

        let processor = ImageProcessor::new(
            dir.path(),
            dir.path().join("dist"),
            "https://example.com",
            &[
                dir.path().join("themes/base/static"),
                dir.path().join("themes/child/static"),
            ],
        );
        let processed = processor
            .resize(&ResizeRequest {
                path: "/images/shared.png".to_string(),
                width: Some(6),
                height: Some(6),
                op: ResizeOperation::Fit,
                format: OutputFormat::Png,
                quality: None,
            })
            .expect("resize should succeed");

        assert_eq!(processed.orig_width, 12);
        assert_eq!(processed.orig_height, 6);
    }

    #[test]
    fn output_file_name_changes_when_arguments_change() {
        let first = build_output_file_name(
            b"abc",
            &ResizeRequest {
                path: "cover.png".to_string(),
                width: Some(10),
                height: Some(10),
                op: ResizeOperation::Fit,
                format: OutputFormat::Auto,
                quality: Some(75),
            },
            OutputFormat::Jpg,
        );
        let second = build_output_file_name(
            b"abc",
            &ResizeRequest {
                path: "cover.png".to_string(),
                width: Some(20),
                height: Some(10),
                op: ResizeOperation::Fit,
                format: OutputFormat::Auto,
                quality: Some(75),
            },
            OutputFormat::Jpg,
        );

        assert_ne!(first, second);
    }
}
