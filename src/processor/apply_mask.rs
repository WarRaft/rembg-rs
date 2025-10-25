use crate::{RembgError, RemovalOptions};
use image::{DynamicImage, ImageBuffer, Luma, Rgba, RgbaImage};
use ndarray::{Array4, Axis};

/// Apply mask (Array4<f32>) to original image to remove background
pub fn apply_mask(
    original: &DynamicImage,
    mask_output: &Array4<f32>,
    options: &RemovalOptions,
) -> crate::Result<RgbaImage> {
    // Convert input image to RGBA for easy alpha manipulation
    let rgba_img = original.to_rgba8();
    let (width, height) = rgba_img.dimensions();

    // Model output: (1, 1, H, W) or (N, C, H, W)
    if mask_output.ndim() != 4 {
        return Err(RembgError::PreprocessingError(format!(
            "Unexpected mask shape: {:?}",
            mask_output.shape()
        )));
    }

    // Extract first batch/channel
    let temp_axis = mask_output.index_axis(Axis(0), 0);
    let mask_data = temp_axis.index_axis(Axis(0), 0);
    let (model_h, model_w) = mask_data.dim();

    // Resize if model output differs from original size
    let need_resize = (model_w as u32 != width) || (model_h as u32 != height);
    let mut mask_gray: ImageBuffer<Luma<u8>, Vec<u8>> =
        ImageBuffer::new(model_w as u32, model_h as u32);

    // Convert float logits → sigmoid → u8 mask (0–255)
    for (x, y, pixel) in mask_gray.enumerate_pixels_mut() {
        let v = mask_data[[y as usize, x as usize]];
        let s = 1.0 / (1.0 + (-v).exp()); // sigmoid
        pixel.0[0] = (s * 255.0).clamp(0.0, 255.0) as u8;
    }

    let mask_resized = if need_resize {
        image::imageops::resize(
            &mask_gray,
            width,
            height,
            image::imageops::FilterType::Lanczos3,
        )
    } else {
        mask_gray
    };

    // --- Теперь та же логика, что и в исходной версии ---
    let mut result = RgbaImage::new(width, height);
    let thr_u8 = options.threshold;
    let thr_f = thr_u8 as f32;

    let smooth_scale = if thr_u8 < 255 {
        Some(255.0 / (255.0 - thr_f))
    } else {
        None
    };

    for (x, y, src) in rgba_img.enumerate_pixels() {
        let mask_value = mask_resized.get_pixel(x, y).0[0];

        let alpha: u8 = if options.binary {
            if mask_value >= thr_u8 { 255 } else { 0 }
        } else {
            match smooth_scale {
                Some(scale) => {
                    let mv = mask_value as f32;
                    ((mv - thr_f) * scale * 255.0).clamp(0.0, 255.0).round() as u8
                }
                None => {
                    if mask_value == 255 {
                        255
                    } else {
                        0
                    }
                }
            }
        };

        result.put_pixel(x, y, Rgba([src.0[0], src.0[1], src.0[2], alpha]));
    }

    Ok(result)
}
