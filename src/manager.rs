use crate::error::RembgError;
use ndarray::Array;
use ort::{
    session::{
        Session,
        builder::{GraphOptimizationLevel, SessionBuilder},
    },
    value::Value,
};
use std::path::Path;

pub struct ModelManager {
    session: Session,
}

impl ModelManager {
    /// Create a new model manager from model file
    ///
    /// Uses memory mapping - OS decides whether to keep model in RAM or load on demand.
    /// This is the most memory-efficient approach for long-running applications.
    pub fn from_file(model_path: &Path) -> Result<Self, RembgError> {
        // Create session with model file (uses memory mapping)
        // In ort 2.0, environment is initialized automatically
        let session = SessionBuilder::new()?
            .with_optimization_level(GraphOptimizationLevel::Level3)?
            .with_intra_threads(4)?
            .commit_from_file(model_path)?;

        Ok(Self { session })
    }

    /// Run inference on preprocessed input
    pub fn run_inference(
        &mut self,
        input: &ndarray::Array4<f32>,
    ) -> Result<ndarray::Array4<f32>, RembgError> {
        // Convert to tuple format (shape, data) which ort accepts
        let shape = input.shape().to_vec();
        let data: Vec<f32> = input.iter().copied().collect();

        // Create input tensor from tuple (shape, data)
        let input_tensor = Value::from_array((shape.as_slice(), data))?;

        // Get input/output names before running inference (to avoid borrow issues)
        let input_name = { String::from(self.session.inputs()[0].name()) };
        let output_name = { String::from(self.session.outputs()[0].name()) };

        // Run inference
        let outputs = self
            .session
            .run(ort::inputs![input_name.as_str() => input_tensor])?;

        // Extract output tensor by name
        let output = outputs
            .get(output_name.as_str())
            .ok_or_else(|| RembgError::TensorError("No output from model".to_string()))?;

        // Extract tensor data - try_extract_tensor returns (shape, data slice)
        let (shape, data) = output
            .try_extract_tensor::<f32>()
            .map_err(|e| RembgError::TensorError(format!("Failed to extract tensor: {}", e)))?;

        // Convert shape from i64 to usize
        let shape_vec: Vec<usize> = shape.as_ref().iter().map(|&x| x as usize).collect();

        // Create ndarray from data
        let output_array =
            Array::from_shape_vec(shape_vec.as_slice(), data.to_vec()).map_err(|e| {
                RembgError::TensorError(format!("Failed to create output array: {}", e))
            })?;

        // Reshape to 4D if needed
        let output_shape = output_array.shape();
        let output_4d = if output_shape.len() == 4 {
            output_array.into_dimensionality()?
        } else if output_shape.len() == 3 {
            output_array
                .insert_axis(ndarray::Axis(0))
                .into_dimensionality()?
        } else if output_shape.len() == 2 {
            // Add batch and channel dimensions
            output_array
                .insert_axis(ndarray::Axis(0))
                .insert_axis(ndarray::Axis(0))
                .into_dimensionality()?
        } else {
            return Err(RembgError::TensorError(format!(
                "Unexpected output shape: {:?}",
                output_shape
            )));
        };

        Ok(output_4d)
    }
}
