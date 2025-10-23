//! Error types for rembg-rs library

use std::fmt;

/// Error types that can occur during background removal operations
#[derive(Debug)]
pub enum RembgError {
    /// Image processing error from the `image` crate
    ImageError(image::ImageError),
    
    /// ONNX Runtime error
    OnnxError(ort::OrtError),
    
    /// I/O error (file operations)
    IoError(std::io::Error),
    
    /// Model file not found
    ModelNotFound(String),
    
    /// Invalid input provided
    InvalidInput(String),
    
    /// Unsupported image format
    UnsupportedFormat(String),
    
    /// Image preprocessing failed
    PreprocessingError(String),
    
    /// Tensor operation failed
    TensorError(String),
    
    /// Array shape error
    ShapeError(String),
}

impl fmt::Display for RembgError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RembgError::ImageError(e) => write!(f, "Image processing error: {}", e),
            RembgError::OnnxError(e) => write!(f, "ONNX Runtime error: {}", e),
            RembgError::IoError(e) => write!(f, "I/O error: {}", e),
            RembgError::ModelNotFound(name) => write!(f, "Model not found: {}", name),
            RembgError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            RembgError::UnsupportedFormat(fmt) => write!(f, "Unsupported image format: {}", fmt),
            RembgError::PreprocessingError(reason) => write!(f, "Image preprocessing failed: {}", reason),
            RembgError::TensorError(op) => write!(f, "Tensor operation failed: {}", op),
            RembgError::ShapeError(msg) => write!(f, "Shape error: {}", msg),
        }
    }
}

impl std::error::Error for RembgError {}

impl From<image::ImageError> for RembgError {
    fn from(err: image::ImageError) -> Self {
        RembgError::ImageError(err)
    }
}

impl From<ort::OrtError> for RembgError {
    fn from(err: ort::OrtError) -> Self {
        RembgError::OnnxError(err)
    }
}

impl From<std::io::Error> for RembgError {
    fn from(err: std::io::Error) -> Self {
        RembgError::IoError(err)
    }
}

impl From<ndarray::ShapeError> for RembgError {
    fn from(err: ndarray::ShapeError) -> Self {
        RembgError::ShapeError(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, RembgError>;