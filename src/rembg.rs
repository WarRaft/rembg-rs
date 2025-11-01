use crate::clean_sticker_border::clean_sticker_border;
use crate::error::RembgError;
use crate::manager::ModelManager;
use crate::options::RemovalOptions;
use crate::result::RemovalResult;
use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, RgbImage, Rgba, RgbaImage};
use ndarray::{Array4, Axis};

pub fn rembg(
    manager: &ModelManager,
    image: DynamicImage,
    options: &RemovalOptions,
) -> Result<RemovalResult, RembgError> {
    let (original_width, original_height) = image.dimensions();

    let preprocessed = {
        // Convert to RGB if not already
        let rgb_img = image.to_rgb8();

        // Resize image
        let target_width = 320;
        let target_height = 320;
        let resized = image::imageops::resize(
            &rgb_img,
            target_width,
            target_height,
            image::imageops::FilterType::Lanczos3,
        );

        // Convert to normalized float array with shape [1, 3, height, width]
        let mut array = Array4::<f32>::zeros((1, 3, target_height as usize, target_width as usize));

        for (x, y, pixel) in resized.enumerate_pixels() {
            let [r, g, b] = pixel.0;

            // Normalize to [0, 1] and then to standardized range for model
            array[[0, 0, y as usize, x as usize]] = (r as f32 / 255.0 - 0.485) / 0.229;
            array[[0, 1, y as usize, x as usize]] = (g as f32 / 255.0 - 0.456) / 0.224;
            array[[0, 2, y as usize, x as usize]] = (b as f32 / 255.0 - 0.406) / 0.225;
        }

        array
    };

    // Run model inference
    let mask_output: Array4<f32> = manager.run_inference(&preprocessed)?;

    // Postprocess the mask
    let mask = {
        use image::imageops::FilterType;

        // gamma for visualization
        let gamma: f32 = 0.5;

        // extract first batch/channel
        let temp_axis = mask_output.index_axis(Axis(0), 0);
        let mask_data = temp_axis.index_axis(Axis(0), 0);
        let (model_height, model_width) = mask_data.dim();

        // build LUT
        let g = gamma.clamp(0.2, 5.0);
        let mut lut = [(0u8, 0u8, 0u8); 256];
        for i in 0..256 {
            let t = (i as f32 / 255.0).powf(g);
            lut[i] = colormap(t);
        }

        // fill heatmap image
        let mut heat = RgbImage::new(model_width as u32, model_height as u32);
        for (x, y, pixel) in heat.enumerate_pixels_mut() {
            let v = mask_data[[y as usize, x as usize]];
            let s = 1.0 / (1.0 + (-v).exp());
            let idx = (s * 255.0).round() as usize;
            let (r, g, b) = lut[idx.min(255)];
            *pixel = image::Rgb([r, g, b]);
        }

        image::imageops::resize(&heat, original_width, original_height, FilterType::Lanczos3)
    };

    // Apply mask to original image
    let result_image = {
        // Convert input image to RGBA
        let rgba_img = image.to_rgba8();
        let (width, height) = rgba_img.dimensions();

        if mask_output.ndim() != 4 {
            return Err(RembgError::PreprocessingError(format!(
                "Unexpected mask shape: {:?}",
                mask_output.shape()
            )));
        }

        let temp_axis = mask_output.index_axis(Axis(0), 0);
        let mask_data = temp_axis.index_axis(Axis(0), 0);
        let (model_h, model_w) = mask_data.dim();

        let need_resize = (model_w as u32 != width) || (model_h as u32 != height);
        let mut mask_gray: ImageBuffer<Luma<u8>, Vec<u8>> =
            ImageBuffer::new(model_w as u32, model_h as u32);

        for (x, y, pixel) in mask_gray.enumerate_pixels_mut() {
            let v = mask_data[[y as usize, x as usize]];
            let s = 1.0 / (1.0 + (-v).exp());
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

        if options.sticker {
            result = clean_sticker_border(&result);
        }

        result
    };

    Ok(RemovalResult {
        image: result_image,
        mask,
    })
}

// --- Inlined preprocessor/processor helpers ---

#[inline]
fn lerp(a: (u8, u8, u8), b: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
    let (ar, ag, ab) = a;
    let (br, bg, bb) = b;
    let r = ar as f32 + (br as f32 - ar as f32) * t;
    let g = ag as f32 + (bg as f32 - ag as f32) * t;
    let b = ab as f32 + (bb as f32 - ab as f32) * t;
    (r.round() as u8, g.round() as u8, b.round() as u8)
}

static STOPS: &[(f32, (u8, u8, u8))] = &[
    (0.00, (0, 0, 0)),
    (0.15, (0, 0, 64)),
    (0.30, (0, 0, 255)),
    (0.45, (128, 0, 192)),
    (0.60, (255, 0, 0)),
    (0.75, (255, 128, 0)),
    (0.90, (255, 255, 0)),
    (1.00, (255, 255, 255)),
];

#[inline]
fn colormap(t: f32) -> (u8, u8, u8) {
    let t = t.clamp(0.0, 1.0);
    for w in STOPS.windows(2) {
        let (t0, c0) = (w[0].0, w[0].1);
        let (t1, c1) = (w[1].0, w[1].1);
        if t <= t1 {
            let local = if t1 > t0 { (t - t0) / (t1 - t0) } else { 0.0 };
            return lerp(c0, c1, local);
        }
    }
    STOPS.last().unwrap().1
}
