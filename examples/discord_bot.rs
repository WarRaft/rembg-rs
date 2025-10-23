//! Discord bot integration example
//!
//! This example shows how to integrate rembg-rs with a Discord bot
//! to process images from user attachments.
//!
//! ## Memory Management
//!
//! The library uses memory-mapped model loading via `Rembg::new(path)`:
//! - Model file is memory-mapped, not fully loaded into RAM
//! - OS decides whether to keep model in RAM or swap to disk
//! - If RAM is available: model stays cached for fast inference
//! - If RAM is low: OS loads only needed parts on demand
//! - Perfect for long-running Discord bots
//! - No manual memory management needed
//!
//! This is much better than loading model bytes into RAM manually,
//! because OS kernel knows better how to manage memory across all processes.

use rembg::{Rembg, RemovalOptions};
use image::load_from_memory;
use std::path::Path;

/// Example function to process image bytes (like from Discord attachment)
fn process_discord_image(
    image_bytes: &[u8],
    rembg: &mut Rembg,
) -> Result<(Vec<u8>, Vec<u8>), Box<dyn std::error::Error>> {
    // Load image from bytes (Discord attachment)
    let img = load_from_memory(image_bytes)?;
    
    // Process with optimal settings for art
    let options = RemovalOptions::new()
        .with_threshold(0.6)
        .with_binary_mode(true);
    
    let result = rembg.remove_background(img, options)?;
    
    // Convert result to PNG bytes
    let mut result_bytes = Vec::new();
    let result_img = image::DynamicImage::ImageRgba8(result.image().clone());
    result_img.write_to(&mut std::io::Cursor::new(&mut result_bytes), image::ImageFormat::Png)?;
    
    // Convert mask to PNG bytes
    let mut mask_bytes = Vec::new();
    let mask_img = image::DynamicImage::ImageLuma8(result.mask().clone());
    mask_img.write_to(&mut std::io::Cursor::new(&mut mask_bytes), image::ImageFormat::Png)?;
    
    Ok((result_bytes, mask_bytes))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("📱 Discord bot integration example\n");
    
    // Load model once at bot startup - OS manages memory
    println!("📦 Initializing Rembg...");
    let mut rembg = Rembg::new(Path::new("models/u2net.onnx"))?;
    println!("✅ Ready to process images\n");
    
    // Simulate processing an image attachment
    println!("📸 Simulating Discord image processing...");
    let test_image = std::fs::read("test_input/download.jpeg")?;
    
    let (result, mask) = process_discord_image(&test_image, &mut rembg)?;
    
    println!("✅ Processed image: {} bytes", result.len());
    println!("✅ Generated mask: {} bytes", mask.len());
    
    // Save for demonstration
    std::fs::write("examples/discord_result.png", result)?;
    std::fs::write("examples/discord_mask.png", mask)?;
    
    println!("\n💡 In a real Discord bot, you would:");
    println!("   1. Download attachment bytes from Discord message");
    println!("   2. Pass bytes to process_discord_image()");
    println!("   3. Send result_bytes back as attachment");
    println!("   4. Optionally send mask_bytes as well");
    
    Ok(())
}
