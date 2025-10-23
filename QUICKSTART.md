# Quick Start Guide - rembg-rs

## For Discord Bot Integration

### 1. Add dependency to Cargo.toml

```toml
[dependencies]
rembg = { git = "https://github.com/WarRaft/rembg-rs" }
image = "0.25"
```

### 2. Initialize once at bot startup

```rust
use rembg::{Rembg, RemovalOptions};
use std::path::Path;

// Create Rembg instance (do this once, not per message!)
// OS manages memory automatically - uses memory mapping
let mut rembg = Rembg::new(Path::new("models/u2net.onnx"))?;
```

**Why this is efficient:**
- Model loads once at startup using memory mapping
- OS automatically manages memory (keeps in RAM if available, swaps if needed)
- No manual memory management required
- Perfect for long-running bots

### 3. Process images from Discord attachments

```rust
use image::load_from_memory;

// Load image from Discord attachment bytes
let img = load_from_memory(&attachment_bytes)?;

// Process with recommended settings for art
let options = RemovalOptions::new()
    .with_threshold(0.6)      // 0.5-0.7 for art
    .with_binary_mode(true);  // Clean cutout

let result = rembg.process_image(img, options)?;

// Convert to bytes for sending back
let mut bytes = Vec::new();
let img = image::DynamicImage::ImageRgba8(result.image().clone());
img.write_to(&mut std::io::Cursor::new(&mut bytes), image::ImageFormat::Png)?;

// Send bytes as Discord attachment
```

### 4. Recommended settings

**For art/anime:**
```rust
RemovalOptions::new()
    .with_threshold(0.6)
    .with_binary_mode(true)
```

**For photos with soft edges:**
```rust
RemovalOptions::new()
    .with_threshold(0.4)
    .with_binary_mode(false)
```

**For portraits:**
Use `u2net_human_seg.onnx` model instead of `u2net.onnx`

## Full Discord Bot Example

See `examples/discord_bot.rs` for complete integration example.

## API Reference

### `Rembg`
- `new(model_bytes: Vec<u8>)` - Initialize with model bytes

### `RemovalOptions`
- `with_threshold(0.0-1.0)` - Set removal threshold
- `with_binary_mode(bool)` - Enable/disable hard cutout

### `RemovalResult`
- `image()` - Get processed RGBA image
- `mask()` - Get grayscale mask
- `into_parts()` - Get (image, mask) tuple

## Models

Download with `./download_model.sh`:

- **u2net.onnx** - Universal, best for art/objects
- **u2net_human_seg.onnx** - For human portraits
- **silueta.onnx** - Fast but less accurate

## Notes

- Load model once, reuse `Rembg` instance for all images
- Models are ~43-176 MB, load from file or embed with `include_bytes!`
- Processing takes ~1-2 seconds per image on modern CPU
- Always use PNG format for output (transparency support)
- Library is completely independent of file system (except for loading model)
