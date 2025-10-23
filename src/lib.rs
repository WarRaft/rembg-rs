//! # rembg-rs
//!
//! A Rust library for removing backgrounds from images using neural networks.
//!
//! This library uses ONNX Runtime to run pretrained U2-Net models for background removal.
//! It supports various image formats (PNG, JPEG, WebP) and provides flexible postprocessing options.
//!
//! ## Features
//!
//! - Remove backgrounds from images using pretrained neural networks
//! - Multiple models available (universal, human segmentation, fast)
//! - Configurable postprocessing (threshold, binary mode)
//! - Save masks separately for debugging
//! - Optional CLI tool (enable `cli` feature)
//!
//! ## Example
//!
//! ```rust,no_run
//! use rembg::{Rembg, RemovalOptions};
//! use image::open;
//! use std::path::Path;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a new background remover with a model
//!     let mut rembg = Rembg::new(Path::new("models/u2net.onnx"))?;
//!     
//!     // Load image
//!     let img = open("input.jpg")?;
//!     
//!     // Configure removal options
//!     let options = RemovalOptions::default()
//!         .with_threshold(0.5)
//!         .with_binary_mode(false);
//!     
//!     // Remove background
//!     let result = rembg.remove_background(img, options)?;
//!     
//!     // Get the result as raw image data
//!     let (image, mask) = result.into_parts();
//!     
//!     Ok(())
//! }
//! ```

mod error;
mod image_processor;
mod model;

pub use error::{RembgError, Result};

use image::{DynamicImage, GenericImageView, ImageBuffer, Luma, RgbaImage};
use image_processor::ImageProcessor;
use model::ModelManager;

/// Main struct for background removal operations
pub struct Rembg {
    model_manager: ModelManager,
}

impl Rembg {
    /// Create a new background remover from ONNX model file.
    ///
    /// Uses memory mapping - OS manages memory automatically.
    /// Best for long-running applications like Discord bots.
    ///
    /// # Arguments
    ///
    /// * `model_path` - Path to ONNX model file
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Model file not found or invalid
    /// - ONNX Runtime initialization fails
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rembg::Rembg;
    /// use std::path::Path;
    ///
    /// // Load model from file - OS handles memory
    /// let mut rembg = Rembg::new(Path::new("models/u2net.onnx"))?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(model_path: &std::path::Path) -> Result<Self> {
        let model_manager = ModelManager::from_file(model_path)?;
        Ok(Self { model_manager })
    }

    /// Remove background from an image.
    ///
    /// This is the main method for processing images. It takes a `DynamicImage`
    /// and returns a result with the processed image and mask.
    ///
    /// # Arguments
    ///
    /// * `image` - The input image
    /// * `options` - Removal options (threshold, binary mode, etc.)
    ///
    /// # Returns
    ///
    /// Returns a `RemovalResult` containing the processed image and mask.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Image processing fails
    /// - Model inference fails
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use rembg::{Rembg, RemovalOptions};
    /// use image::open;
    ///
    /// let mut rembg = Rembg::new("models/u2net.onnx")?;
    /// let img = open("input.jpg")?;
    /// let result = rembg.remove_background(img, RemovalOptions::default())?;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn remove_background(
        &mut self,
        image: DynamicImage,
        options: &RemovalOptions,
    ) -> Result<RemovalResult> {
        let (original_width, original_height) = image.dimensions();

        // Preprocess image for model input (320x320 is standard for U2-Net)
        let preprocessed = ImageProcessor::preprocess_for_model(&image, 320, 320)?;

        // Run model inference
        let mask_output = self.model_manager.run_inference(&preprocessed)?;

        // Postprocess the mask
        let mask = ImageProcessor::postprocess_mask(mask_output, original_width, original_height)?;

        // Apply mask to original image
        let result_image = ImageProcessor::apply_mask(&image, &mask, &options)?;

        Ok(RemovalResult {
            image: result_image,
            mask,
        })
    }
}

/// Options for background removal
#[derive(Debug, Clone)]
pub struct RemovalOptions {
    /// Threshold for alpha matting (0–255).
    /// Higher values = more aggressive background removal.
    /// - 76–102: Soft edges with semi-transparency (≈0.3–0.4)
    /// - 128: Balanced (default, ≈0.5)
    /// - 153–179: Stronger cutout, cleaner edges (≈0.6–0.7)
    pub threshold: u8,

    /// If true, creates hard cutout without semi-transparency.
    /// If false, allows soft edges for more natural blending.
    pub binary: bool,
}

impl Default for RemovalOptions {
    fn default() -> Self {
        Self {
            threshold: 160,
            binary: false,
        }
    }
}

impl RemovalOptions {
    /// Create new options with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the threshold value (0-255)
    pub fn with_threshold(mut self, threshold: u8) -> Self {
        self.threshold = threshold.clamp(0, 255);
        self
    }

    /// Enable or disable binary mode
    pub fn with_binary_mode(mut self, binary: bool) -> Self {
        self.binary = binary;
        self
    }
}

/// Result of background removal operation
pub struct RemovalResult {
    /// The image with background removed (RGBA)
    pub image: RgbaImage,
    /// The mask used for removal (grayscale, 0-255)
    pub mask: ImageBuffer<Luma<u8>, Vec<u8>>,
}

impl RemovalResult {
    /// Get a reference to the result image (RGBA with transparent background)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use rembg::{Rembg, RemovalOptions};
    /// # use image::open;
    /// # let mut rembg = Rembg::new("models/u2net.onnx")?;
    /// # let img = open("input.jpg")?;
    /// let result = rembg.remove_background(img, RemovalOptions::default())?;
    /// let image = result.image();
    /// println!("Image size: {}x{}", image.width(), image.height());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn image(&self) -> &RgbaImage {
        &self.image
    }

    /// Get a reference to the mask (grayscale, 0-255)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use rembg::{Rembg, RemovalOptions};
    /// # use image::open;
    /// # let mut rembg = Rembg::new("models/u2net.onnx")?;
    /// # let img = open("input.jpg")?;
    /// let result = rembg.remove_background(img, RemovalOptions::default())?;
    /// let mask = result.mask();
    /// println!("Mask size: {}x{}", mask.width(), mask.height());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn mask(&self) -> &ImageBuffer<Luma<u8>, Vec<u8>> {
        &self.mask
    }

    /// Consume the result and return the image and mask as owned values
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use rembg::{Rembg, RemovalOptions};
    /// # use image::open;
    /// # let mut rembg = Rembg::new("models/u2net.onnx")?;
    /// # let img = open("input.jpg")?;
    /// let result = rembg.remove_background(img, RemovalOptions::default())?;
    /// let (image, mask) = result.into_parts();
    /// // Now you own the image and mask and can use them as needed
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn into_parts(self) -> (RgbaImage, ImageBuffer<Luma<u8>, Vec<u8>>) {
        (self.image, self.mask)
    }
}
