// CLI module is optional and compiled only when `cli` feature is enabled
#[cfg(feature = "cli")]
pub mod cli;
pub mod error;
pub mod manager;
pub mod options;
pub mod rembg;
pub mod result;
