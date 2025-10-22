# rembg-rs

A Rust utility for removing backgrounds from art-style images using neural networks.

## 🚧 Project Status

✅ **Working!** The project is functional and ready for use on macOS.

## 📋 Requirements

- Rust 1.70+
- ONNX Runtime (installed automatically via `ort` crate)
- macOS (tested on macOS)

## 🏗️ Architecture

The project consists of the following modules:

- `cli.rs` - Command-line argument parsing
- `error.rs` - Error types (without thiserror/anyhow)
- `image_processor.rs` - Image loading, preprocessing and postprocessing
- `model.rs` - ONNX model operations
- `background_removal.rs` - Main module that combines all components

## 🎯 Using Pretrained Models

Pretrained ONNX models are used for background removal.

### Download Models:

```bash
./download_model.sh
```

The script will download all available models to the `models/` directory:

1. **U2-Net** (universal) - `models/u2net.onnx` (~176 MB)
2. **U2-Net Human Segmentation** (for portraits) - `models/u2net_human_seg.onnx` (~176 MB)
3. **Silueta** (fast, lightweight) - `models/silueta.onnx` (~43 MB)

## 🔧 Build

```bash
cargo build --release
```

## 🚀 Usage

Use the wrapper script that sets up library paths correctly:

```bash
# Basic usage
./rembg-rs -i input.jpg -o output.png -m models/u2net.onnx

# With JPEG quality setting
./rembg-rs -i input.jpg -o output.jpg -m models/u2net.onnx -q 95
```

Or set the library path manually:

```bash
DYLD_LIBRARY_PATH=./target/release ./target/release/rembg-rs -i input.jpg -o output.png -m models/u2net.onnx
```

## 🧪 Testing

1. Download models:
```bash
./download_model.sh
```

2. Add test images to `test_input/` directory (supports jpg, jpeg, png formats)

3. Run the test script (it will process all images in test_input/):
```bash
./test.sh
```

The script will:
- Clean the `test_output/` directory
- Process all images from `test_input/`
- Save results with `_no_bg.png` suffix in `test_output/`

Results will be saved as PNG files with transparent background.

## 📝 TODO

- [ ] Add batch processing support
- [ ] Optimize performance
- [ ] Add tests
- [ ] Add usage examples
- [ ] Support for other platforms (Linux, Windows)

## 📄 License

MIT License - see [LICENSE](LICENSE) file