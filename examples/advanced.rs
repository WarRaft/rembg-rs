//! Advanced example showing custom image processing
//!
//! This example demonstrates:
//! - Loading an image directly
//! - Processing with custom options
//! - Accessing raw image and mask data
//! - Using binary mode for clean cutouts

use rembg::{Rembg, RemovalOptions};
use image::{open, GenericImageView};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Advanced rembg-rs example\n");

    // Load model from file
    println!("📦 Loading model...");
    let mut rembg = Rembg::new(Path::new("models/u2net.onnx"))?;
    println!("✅ Model loaded\n");

    // Load image manually
    println!("📂 Loading image...");
    let img = open("test_input/download.jpeg")?;
    println!("   Image size: {:?}", img.dimensions());

    // Process with aggressive settings for clean cutout
    println!("\n🖼️  Processing with binary mode...");
    let options = RemovalOptions::new()
        .with_threshold(0.6)      // Higher threshold
        .with_binary_mode(true);  // Clean cutout, no transparency

    let result = rembg.remove_background(img, options)?;
    println!("✅ Processing complete\n");

    // Access the raw image data
    let (image, mask) = result.into_parts();
    println!("📊 Result image: {}x{}", image.width(), image.height());
    println!("📊 Mask: {}x{}", mask.width(), mask.height());

    // You can now use image and mask directly
    // For example, in Discord bot you would send these as attachments
    println!("\n💡 Image and mask are now available for further processing");
    println!("   (e.g., sending to Discord, further editing, etc.)");

    Ok(())
}
