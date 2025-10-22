use crate::error::{RembgError, Result};
use crate::image_processor::ImageProcessor;
use crate::model::ModelManager;
use std::path::Path;

pub struct BackgroundRemover {
    model_manager: ModelManager,
}

impl BackgroundRemover {
    /// Create a new background remover
    pub fn new(model_manager: ModelManager) -> Result<Self> {
        Ok(Self { model_manager })
    }

    /// Remove background from an image
    pub fn remove_background<P: AsRef<Path>>(
        &mut self,
        input_path: P,
        output_path: P,
        quality: u8,
    ) -> Result<()> {
        let input_path = input_path.as_ref();
        let output_path = output_path.as_ref();

        println!("🖼️  Processing image...");

        // Validate input file exists
        if !input_path.exists() {
            return Err(RembgError::InvalidInput(
                format!("Input file does not exist: {:?}", input_path)
            ));
        }

        // Load the original image
        println!("📂 Loading image...");
        let original_image = ImageProcessor::load_image(input_path)?;
        let (original_width, original_height) = ImageProcessor::get_dimensions(&original_image);
        println!("   Image size: {}x{}", original_width, original_height);

        // Preprocess image for model input (320x320 is standard for U2-Net)
        println!("🔧 Preprocessing image...");
        let preprocessed = ImageProcessor::preprocess_for_model(
            &original_image,
            320,
            320,
        )?;

        // Run model inference
        let mask_output = self.model_manager.run_inference(&preprocessed)?;

        // Postprocess the mask
        println!("🎭 Postprocessing mask...");
        let mask = ImageProcessor::postprocess_mask(
            mask_output,
            original_width,
            original_height,
        )?;

        // Apply mask to original image
        println!("✂️  Applying mask...");
        let result_image = ImageProcessor::apply_mask(&original_image, &mask)?;

        // Save the result
        println!("💾 Saving result...");
        ImageProcessor::save_image(&result_image, output_path, quality)?;

        println!();
        Ok(())
    }
}
