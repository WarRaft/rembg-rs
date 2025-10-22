use crate::error::{RembgError, Result};
use ndarray::{Array, IxDyn};
use ort::{Environment, GraphOptimizationLevel, SessionBuilder};
use std::path::Path;

pub struct ModelManager {
    session: ort::Session,
}

impl ModelManager {
    /// Create a new model manager and load the model
    pub fn new(model_path: &str) -> Result<Self> {
        let model_path = Path::new(model_path);
        
        if !model_path.exists() {
            return Err(RembgError::ModelNotFound(
                format!("Model file not found: {:?}", model_path)
            ));
        }

        println!("📦 Loading ONNX model: {:?}", model_path);

        // Initialize ONNX Runtime environment  
        let environment = Environment::builder()
            .with_name("rembg-rs")
            .with_log_level(ort::LoggingLevel::Warning)
            .build()?
            .into_arc();

        // Create session with the model file
        let session = SessionBuilder::new(&environment)?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .with_model_from_file(model_path)?;

        println!("✅ Model loaded successfully");

        Ok(Self { session })
    }

    /// Run inference on preprocessed input
    pub fn run_inference(&self, input: &ndarray::Array4<f32>) -> Result<ndarray::Array4<f32>> {
        println!("🔄 Running model inference...");
        
        // Convert to dynamic dimensions with CowArray
        let input_shape: Vec<usize> = input.shape().to_vec();
        let input_data: Vec<f32> = input.iter().copied().collect();
        let input_array = Array::from_shape_vec(IxDyn(&input_shape), input_data)
            .map_err(|e| RembgError::TensorError(format!("Failed to create input array: {}", e)))?;
        
        // Create CowArray for ORT
        let input_cow = ndarray::CowArray::from(input_array.view());

        // Create input tensor
        let input_tensor = ort::Value::from_array(self.session.allocator(), &input_cow)?;

        // Run inference
        let outputs = self.session.run(vec![input_tensor])?;

        // Extract output tensor
        let output = outputs
            .get(0)
            .ok_or_else(|| RembgError::TensorError(
                "No output from model".to_string()
            ))?;

        // Convert back to ndarray
        let output_array = output.try_extract::<f32>()?.view().to_owned();
        
        // Reshape to 4D if needed
        let output_shape = output_array.shape();
        let output_4d = if output_shape.len() == 4 {
            output_array.into_dimensionality()?
        } else if output_shape.len() == 3 {
            output_array.insert_axis(ndarray::Axis(0)).into_dimensionality()?
        } else if output_shape.len() == 2 {
            // Add batch and channel dimensions
            output_array
                .insert_axis(ndarray::Axis(0))
                .insert_axis(ndarray::Axis(0))
                .into_dimensionality()?
        } else {
            return Err(RembgError::TensorError(
                format!("Unexpected output shape: {:?}", output_shape)
            ));
        };

        println!("✅ Inference completed");
        Ok(output_4d)
    }
}
