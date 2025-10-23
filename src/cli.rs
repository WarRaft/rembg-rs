use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "rembg-rs",
    about = "A Rust utility for removing backgrounds from art-style images using neural networks",
    version = env!("CARGO_PKG_VERSION"),
    author = "WarRaft"
)]
pub struct Args {
    /// Input image file path
    #[arg(
        short = 'i',
        long = "input",
        help = "Path to the input image file"
    )]
    pub input: PathBuf,

    /// Output image file path
    #[arg(
        short = 'o',
        long = "output",
        help = "Path where the output image will be saved"
    )]
    pub output: PathBuf,

    /// Model file path (ONNX format)
    #[arg(
        short = 'm',
        long = "model",
        default_value = "u2net.onnx",
        help = "Path to the ONNX model file"
    )]
    pub model: String,

    /// Quality for JPEG output (1-100)
    #[arg(
        short = 'q',
        long = "quality",
        default_value = "95",
        help = "JPEG quality (1-100, only applicable for JPEG output)"
    )]
    pub quality: u8,

    /// Alpha matting threshold (0.0-1.0)
    #[arg(
        short = 't',
        long = "threshold",
        default_value = "0.5",
        help = "Threshold for alpha matting (0.0-1.0). Higher values = more aggressive removal"
    )]
    pub threshold: f32,

    /// Enable binary mask mode (no transparency, just cut out)
    #[arg(
        short = 'b',
        long = "binary",
        help = "Use binary mask (full transparent or opaque, no semi-transparency)"
    )]
    pub binary: bool,

    /// Save mask as separate file
    #[arg(
        short = 's',
        long = "save-mask",
        help = "Save the mask as a separate grayscale image alongside the output"
    )]
    pub save_mask: bool,
}