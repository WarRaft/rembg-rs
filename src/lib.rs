pub mod cli;
mod error;
pub mod manager;
pub mod options;
pub mod processor;
pub mod rembg;
pub mod result;

pub use error::{RembgError, Result};

use crate::options::RemovalOptions;
