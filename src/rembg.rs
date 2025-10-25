use crate::manager::ModelManager;
use crate::options::RemovalOptions;
use crate::processor::apply_mask::apply_mask;
use crate::processor::postprocess_mask::postprocess_mask;
use crate::result::RemovalResult;
use image::{DynamicImage, GenericImageView};
use ndarray::Array4;

pub fn rembg(
    manager: ModelManager,
    image: DynamicImage,
    options: &RemovalOptions,
) -> crate::Result<RemovalResult> {
    let (original_width, original_height) = image.dimensions();

    println!(
        "Original image size: {}x{}",
        original_width, original_height
    );
    println!("Options: {:?}", options);

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
    let mask = postprocess_mask(&mask_output, original_width, original_height)?;

    // Apply mask to original image
    let result_image = apply_mask(&image, &mask_output, &options)?;

    Ok(RemovalResult {
        image: result_image,
        mask,
    })
}
