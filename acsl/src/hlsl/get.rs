use crate::HlslProgram;

impl HlslProgram {
    /// Get the content of an [`HlslProgram`]
    pub fn content(&self) -> &str {
        &self.content
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
