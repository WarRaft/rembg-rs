use crate::RemovalOptions;
use crate::error::{RembgError, Result};
use image::{DynamicImage, ImageBuffer, Rgb, Rgba, RgbaImage, imageops::FilterType};
use ndarray::{Array4, Axis};
use std::path::Path;

pub struct ImageProcessor;

impl ImageProcessor {
    /// Load an image from file path
    #[allow(dead_code)]
    pub fn load_image<P: AsRef<Path>>(path: P) -> Result<DynamicImage> {
        let img = image::open(path)?;
        Ok(img)
    }

    /// Preprocess image for neural network input
    /// Resize to target size and normalize pixel values
    pub fn preprocess_for_model(
        img: &DynamicImage,
        target_width: u32,
        target_height: u32,
    ) -> Result<Array4<f32>> {
        // Convert to RGB if not already
        let rgb_img = img.to_rgb8();

        // Resize image
        let resized =
            image::imageops::resize(&rgb_img, target_width, target_height, FilterType::Lanczos3);

        // Convert to normalized float array with shape [1, 3, height, width]
        let mut array = Array4::<f32>::zeros((1, 3, target_height as usize, target_width as usize));

        for (x, y, pixel) in resized.enumerate_pixels() {
            let [r, g, b] = pixel.0;

            // Normalize to [0, 1] and then to [-1, 1] for some models
            array[[0, 0, y as usize, x as usize]] = (r as f32 / 255.0 - 0.485) / 0.229;
            array[[0, 1, y as usize, x as usize]] = (g as f32 / 255.0 - 0.456) / 0.224;
            array[[0, 2, y as usize, x as usize]] = (b as f32 / 255.0 - 0.406) / 0.225;
        }

        Ok(array)
    }

    /// Postprocess model output to create mask
    pub fn postprocess_mask(
        mask_output: Array4<f32>,
        original_width: u32,
        original_height: u32,
    ) -> Result<ImageBuffer<image::Luma<u8>, Vec<u8>>> {
        let temp_axis = mask_output.index_axis(Axis(0), 0);
        let mask_data = temp_axis.index_axis(Axis(0), 0);

        let (model_height, model_width) = mask_data.dim();

        // Create grayscale image from mask
        let mut mask_img = image::GrayImage::new(model_width as u32, model_height as u32);

        for (x, y, pixel) in mask_img.enumerate_pixels_mut() {
            let mask_value = mask_data[[y as usize, x as usize]];
            // Apply sigmoid and convert to 0-255 range
            let normalized = 1.0 / (1.0 + (-mask_value).exp());
            pixel.0[0] = (normalized * 255.0) as u8;
        }

        // Resize mask to original image size
        let resized_mask = image::imageops::resize(
            &mask_img,
            original_width,
            original_height,
            FilterType::Lanczos3,
        );

        Ok(resized_mask)
    }

    /// Apply mask to original image to remove background
    pub fn apply_mask(
        original: &DynamicImage,
        mask: &ImageBuffer<image::Luma<u8>, Vec<u8>>,
        options: &RemovalOptions,
    ) -> Result<RgbaImage> {
        // Convert input image to RGBA for easy alpha manipulation
        let rgba_img = original.to_rgba8();
        let (width, height) = rgba_img.dimensions();

        // Check that mask and image have the same dimensions
        if mask.dimensions() != (width, height) {
            return Err(RembgError::PreprocessingError(
                "Mask dimensions don't match original image".to_string(),
            ));
        }

        let mut result = RgbaImage::new(width, height);
        let thr_u8 = options.threshold;
        let thr_f = thr_u8 as f32;

        // Precompute scaling factor for smooth mode:
        // alpha = (mask - threshold) * (255 / (255 - threshold))
        // This avoids division in every loop iteration.
        let smooth_scale = if thr_u8 < 255 {
            Some(255.0 / (255.0 - thr_f))
        } else {
            None // cannot divide if threshold == 255
        };

        // Iterate over each pixel
        for (x, y, src) in rgba_img.enumerate_pixels() {
            let mask_value = mask.get_pixel(x, y).0[0];

            // Compute alpha depending on the mode
            let alpha: u8 = if options.binary {
                // Binary mode:
                // If mask >= threshold → fully opaque
                // Otherwise → fully transparent
                if mask_value >= thr_u8 { 255 } else { 0 }
            } else {
                // Smooth mode:
                match smooth_scale {
                    Some(scale) => {
                        // Linear normalization above the threshold
                        // Values below threshold → 0
                        // Values above → scaled to 0–255 range
                        let mv = mask_value as f32;
                        let a = ((mv - thr_f) * scale * 255.0).clamp(0.0, 255.0).round() as u8;
                        a
                    }
                    None => {
                        // Special case when threshold == 255:
                        // Only pure white pixels stay visible
                        if mask_value == 255 { 255 } else { 0 }
                    }
                }
            };

            // Write the resulting RGBA pixel with computed alpha
            result.put_pixel(x, y, Rgba([src.0[0], src.0[1], src.0[2], alpha]));
        }

        Ok(result)
    }

    /// Save the processed image
    #[allow(dead_code)]
    pub fn save_image<P: AsRef<Path>>(img: &RgbaImage, path: P, quality: u8) -> Result<()> {
        let path = path.as_ref();

        match path.extension().and_then(|s| s.to_str()) {
            Some("png") => {
                img.save(path)?;
            }
            Some("jpg") | Some("jpeg") => {
                // Convert RGBA to RGB for JPEG
                let rgb_img = ImageBuffer::from_fn(img.width(), img.height(), |x, y| {
                    let rgba = img.get_pixel(x, y);
                    let alpha = rgba.0[3] as f32 / 255.0;

                    // Blend with white background
                    let r = ((rgba.0[0] as f32 * alpha) + (255.0 * (1.0 - alpha))) as u8;
                    let g = ((rgba.0[1] as f32 * alpha) + (255.0 * (1.0 - alpha))) as u8;
                    let b = ((rgba.0[2] as f32 * alpha) + (255.0 * (1.0 - alpha))) as u8;

                    Rgb([r, g, b])
                });

                let mut output = std::fs::File::create(path)?;
                let encoder =
                    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, quality);
                rgb_img.write_with_encoder(encoder)?;
            }
            Some("webp") => {
                img.save(path)?;
            }
            Some(ext) => {
                return Err(RembgError::UnsupportedFormat(ext.to_string()));
            }
            None => {
                return Err(RembgError::UnsupportedFormat("unknown".to_string()));
            }
        }

        Ok(())
    }

    /// Save mask as RGBA image with alpha channel (mask value becomes alpha)
    #[allow(dead_code)]
    pub fn save_mask(
        mask: &ImageBuffer<image::Luma<u8>, Vec<u8>>,
        path: &Path,
        quality: u8,
    ) -> Result<()> {
        let (width, height) = mask.dimensions();

        // Create RGBA image where mask value becomes alpha channel
        // White in mask = fully opaque white, Black in mask = fully transparent
        let mut rgba_img = RgbaImage::new(width, height);

        for (x, y, pixel) in mask.enumerate_pixels() {
            let alpha = pixel.0[0]; // Use mask value as alpha
            rgba_img.put_pixel(x, y, Rgba([255, 255, 255, alpha]));
        }

        let img = DynamicImage::ImageRgba8(rgba_img);

        // Save based on extension
        let extension = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_lowercase());

        match extension.as_deref() {
            Some("png") => {
                img.save(path)?;
            }
            Some("jpg") | Some("jpeg") => {
                // JPEG doesn't support transparency, convert to RGB with white background
                let rgb_img = img.to_rgb8();
                let mut output = std::fs::File::create(path)?;
                let encoder =
                    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut output, quality);
                rgb_img.write_with_encoder(encoder)?;
            }
            Some("webp") => {
                img.save(path)?;
            }
            Some(ext) => {
                return Err(RembgError::UnsupportedFormat(ext.to_string()));
            }
            None => {
                return Err(RembgError::UnsupportedFormat("unknown".to_string()));
            }
        }

        Ok(())
    }
}
