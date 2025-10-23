# rembg-rs

A Rust library for removing backgrounds from images using neural networks (ONNX Runtime + U2-Net).

## � Features

- 🎨 Remove backgrounds from images using pretrained neural networks
- 🔧 Flexible API for library and CLI usage
- 📦 Multiple models available (universal, human segmentation, fast)
- ⚙️ Configurable postprocessing (threshold, binary mode)
- 🎭 Export masks separately
- 🖼️ Support for PNG, JPEG, WebP formats
- 📚 Well-documented English API

## 📋 Requirements

- Rust 1.70+
- ONNX Runtime (installed automatically via `ort` crate)

## 🏗️ Installation

### As a Library (for Discord bots, web services, etc.)

Add to your `Cargo.toml`:

```toml
[dependencies]
rembg = { git = "https://github.com/WarRaft/rembg-rs" }
```

### As a CLI Tool

```toml
[dependencies]
rembg = { git = "https://github.com/WarRaft/rembg-rs", features = ["cli"] }
```

Or build from source:

```bash
cargo build --release --features cli
```

## 📖 Library Usage

### Basic Example

```rust
use rembg::{Rembg, RemovalOptions};
use image::open;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new background remover - OS manages memory
    let mut rembg = Rembg::new(Path::new("models/u2net.onnx"))?;
    
    // Load image
    let img = open("input.jpg")?;
    
    // Configure removal options
    let options = RemovalOptions::default()
        .with_threshold(0.5)
        .with_binary_mode(false);
    
    // Remove background
    let result = rembg.remove_background(img, options)?;
    
    // Get the result as raw image data
    let (image, mask) = result.into_parts();
    
    // Now you can use image and mask
    // (save to file, send over network, etc.)
    
    Ok(())
}
```

### Advanced Example (for Discord bots)

```rust
use rembg::{Rembg, RemovalOptions};
use image::load_from_memory;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize once at bot startup - OS manages memory automatically
    let mut rembg = Rembg::new(Path::new("models/u2net.onnx"))?;
    
    // Process Discord attachment bytes
    let image_bytes = download_from_discord(); // your function
    let img = load_from_memory(&image_bytes)?;
    
    // Process with custom options
    let options = RemovalOptions::new()
        .with_threshold(0.6)
        .with_binary_mode(true);
    
    let result = rembg.remove_background(img, options)?;
    
    // Get raw image and mask for further processing
    let (image, mask) = result.into_parts();
    
    // Convert to bytes and send back to Discord
    // See examples/discord_bot.rs for complete example
    
    Ok(())
}
```

### Memory Management

The library uses **memory-mapped model loading** - OS automatically manages memory:
- If you have enough RAM, model stays in memory for fast inference
- If RAM is low, OS can swap model to disk and load on demand
- Perfect for long-running Discord bots
- No manual memory management needed

See [examples/](examples/) for more usage examples.

## 🎯 Models

### Download Models

```bash
./download_model.sh
```

Available models:

1. **u2net.onnx** (~176 MB) - Universal model for any images, best for art
2. **u2net_human_seg.onnx** (~176 MB) - Specialized for human portraits
3. **silueta.onnx** (~43 MB) - Fast and lightweight

## �️ CLI Usage

```bash
# Basic usage
./rembg-rs -i input.jpg -o output.png -m models/u2net.onnx

# With custom threshold (0.0-1.0)
./rembg-rs -i input.jpg -o output.png -t 0.6

# Binary mode (clean cutout, no semi-transparency)
./rembg-rs -i input.jpg -o output.png -b

# Save mask alongside result
./rembg-rs -i input.jpg -o output.png -s
```

### CLI Parameters

- `-i, --input <PATH>` - Input image path
- `-o, --output <PATH>` - Output image path  
- `-m, --model <PATH>` - Model file path (default: u2net.onnx)
- `-t, --threshold <0.0-1.0>` - Threshold (default: 0.5)
  - 0.3-0.4: Soft edges
  - 0.5: Balanced
  - 0.6-0.7: Clean cutout
- `-b, --binary` - Binary mode (no semi-transparency)
- `-s, --save-mask` - Save mask as separate file
- `-q, --quality <1-100>` - JPEG quality (default: 95)

## 🧪 Testing

1. Download models:
```bash
./download_model.sh
```

2. Add test images to `test_input/` directory

3. Run the test script:
```bash
./test.sh
```

## 📚 API Documentation

### `Rembg`

Main struct for background removal operations.

**Methods:**
- `new(model_path: &str) -> Result<Self>` - Create new instance with ONNX model
- `remove_background(image, options) -> Result<RemovalResult>` - Process DynamicImage

### `RemovalOptions`

Configuration for background removal.

**Fields:**
- `threshold: f32` - Alpha matting threshold (0.0-1.0)
- `binary: bool` - Binary mode (hard cutout vs soft edges)

**Methods:**
- `default()` - Create with default values (threshold: 0.5, binary: false)
- `with_threshold(f32)` - Set threshold
- `with_binary_mode(bool)` - Set binary mode

### `RemovalResult`

Result of background removal.

**Fields:**
- `image: RgbaImage` - Processed image with transparent background
- `mask: GrayImage` - Mask used for removal (0-255)

**Methods:**
- `image()` - Get reference to RGBA image
- `mask()` - Get reference to grayscale mask
- `into_parts()` - Consume and return (image, mask)

Note: File I/O operations are not part of the library core API.
Use the `image` crate to load/save images as needed.

## 🏗️ Architecture

### Memory Management

The library uses **memory-mapped model loading** for optimal memory efficiency:

```rust
// Model is memory-mapped, not loaded into RAM
let mut rembg = Rembg::new("models/u2net.onnx")?;
```

**How it works:**
- Model file is mapped into virtual memory (not loaded into RAM)
- OS kernel decides whether to cache pages in physical RAM
- If RAM is available: frequently used pages stay cached
- If RAM is low: OS swaps pages as needed
- No manual memory management required

**Usage:**
```rust
use std::path::Path;

// Simple and efficient
let mut rembg = Rembg::new(Path::new("models/u2net.onnx"))?;
```

**Benefits for Discord bots:**
- ✅ Model loaded once at startup
- ✅ OS automatically optimizes memory usage
- ✅ Multiple bots can share same model file (copy-on-write)
- ✅ No memory spikes during processing
- ✅ Efficient for long-running processes

**Memory usage:**
- Model file size: ~176 MB (u2net) or ~43 MB (silueta)
- Actual RAM usage: Managed by OS based on available memory
- Per-image processing: ~20-30 MB temporary buffers

### Library Structure

- `lib.rs` - Public API and main types
- `error.rs` - Error types with detailed documentation
- `image_processor.rs` - Image loading, preprocessing, postprocessing
- `model.rs` - ONNX Runtime integration
- `cli.rs` - CLI interface (optional, requires `cli` feature)
- `main.rs` - CLI entry point (requires `cli` feature)

### Processing Pipeline

1. **Load Image** - Read from file or use existing DynamicImage
2. **Preprocess** - Resize to 320x320, normalize to [-1, 1]
3. **Inference** - Run U2-Net model via ONNX Runtime
4. **Postprocess Mask** - Apply sigmoid, resize back to original size
5. **Apply Mask** - Apply threshold and binary mode, create RGBA output
6. **Save** - Export result and optionally mask

## 📦 Publishing

This library is designed to be used as a git dependency or published to crates.io.

### Features
- `default = []` - Library only, no CLI
- `cli` - Enables command-line interface

## 🙏 Credits

- [danielgatis/rembg](https://github.com/danielgatis/rembg) - Original Python implementation and pretrained models
- [U2-Net](https://github.com/xuebinqin/U-2-Net) - Neural network architecture

## 📄 License

MIT License - see [LICENSE](LICENSE) file