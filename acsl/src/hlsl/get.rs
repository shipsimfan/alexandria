use crate::{HlslProgram, InputLayout};

impl HlslProgram {
    /// Get the content of an [`HlslProgram`]
    pub fn content(&self) -> &str {
        &self.content
    }

    /// Get the input layout describing the vertices
    pub fn input_layout(&self) -> &InputLayout {
        &self.input_layout
    }

    /// Get the name of the vertex shader entry point
    pub fn vertex_entry(&self) -> &str {
        &self.vertex_entry
    }

    /// Get the name of the pixel shader entry point
    pub fn pixel_entry(&self) -> &str {
        &self.pixel_entry
    }
}
