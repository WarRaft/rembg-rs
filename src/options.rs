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
