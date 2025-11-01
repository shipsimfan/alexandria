use crate::graphics::{InputElement, InputElementType};

impl InputElement {
    /// Get the semantic name assigned to the variable on the GPU
    pub fn semantic_name(&self) -> &'static str {
        self.semantic_name
    }

    /// Get the semantic index assigned to the variable on the GPU
    pub fn semantic_index(&self) -> u8 {
        self.semantic_index
    }

    /// Get the type of the input element on the GPU
    pub fn r#type(&self) -> InputElementType {
        self.r#type
    }
}
