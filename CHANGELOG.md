# Changelog

All notable changes to this project will be documented in this file.

## [0.1.0] - 2025-10-23

### Added
- ✨ Initial release
- 🎨 Background removal using U2-Net ONNX models
- 📚 Well-documented English API
- 🔧 Configurable postprocessing (threshold, binary mode)
- 🎭 Export masks separately
- 🖼️ Support for PNG, JPEG, WebP formats
- 📦 Multiple pretrained models (u2net, u2net_human_seg, silueta)
- 🖥️ Optional CLI tool (feature flag)
- 📖 Comprehensive examples and documentation
- 🤖 Discord bot integration example
- 💾 Memory-mapped model loading for optimal memory management

### Features
- Library-first design with optional CLI
- `Rembg::new(path)` - Memory-mapped model loading, OS manages memory automatically
- `RemovalOptions` - Configuration builder
- `RemovalResult` - Result with image and mask access
- Custom error types with detailed messages
- No external logging or error handling crates
- Efficient memory usage for long-running applications (Discord bots)

### Examples
- `basic.rs` - Simple usage example
- `advanced.rs` - Custom image processing
- `discord_bot.rs` - Discord bot integration

### Documentation
- README.md - Full project documentation
- QUICKSTART.md - Quick reference for Discord bots
- API docs with extensive examples
- Test scripts for easy verification
