//! Basic example of using rembg-rs library
//!
//! This example demonstrates:
//! - Creating a Rembg instance with a model
//! - Loading an image
//! - Processing with default options
//! - Saving the result and mask

use rembg::{Rembg, RemovalOptions};
use image::{open, DynamicImage};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Basic rembg-rs example\n");

    // Load model from file
    println!("📦 Loading model...");
    let mut rembg = Rembg::new(Path::new("models/u2net.onnx"))?;
    println!("✅ Model loaded\n");

    // Load image
    println!("📂 Loading image...");
    let img = open("test_input/download.jpeg")?;
    println!("✅ Image loaded\n");

    // Configure removal options
    let options = RemovalOptions::default()
        .with_threshold(0.5)
        .with_binary_mode(false);

    // Remove background
    println!("🖼️  Processing image...");
    let result = rembg.remove_background(img, options)?;
    println!("✅ Processing complete\n");

    // Save the result
    println!("💾 Saving result...");
    let result_img = DynamicImage::ImageRgba8(result.image().clone());
    result_img.save("examples/output.png")?;
    println!("✅ Saved to: examples/output.png");

    // Save the mask
    println!("🎭 Saving mask...");
    let mask_img = DynamicImage::ImageLuma8(result.mask().clone());
    mask_img.save("examples/mask.png")?;
    println!("✅ Saved to: examples/mask.png");

    println!("\n✨ Done!");

    Ok(())
}
