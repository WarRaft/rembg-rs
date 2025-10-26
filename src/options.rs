use derive_builder::Builder;

/// Options for background removal
#[derive(Debug, Clone, Builder)]
#[builder(setter(into), default)]
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

    pub sticker: bool,
}

impl Default for RemovalOptions {
    fn default() -> Self {
        Self {
            threshold: 160,
            binary: false,
            sticker: false,
        }
    }
}
