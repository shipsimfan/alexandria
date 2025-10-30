use crate::{D3DProgram, InputLayout};

impl<'a> D3DProgram<'a> {
    /// Gets the content of the vertex shader
    pub fn vertex_content(&self) -> &[u8] {
        &self.vertex_content
    }

    /// Gets the content of the pixel shader
    pub fn pixel_content(&self) -> &[u8] {
        &self.pixel_content
    }

    /// Gets the input layout for this program
    pub fn input_layout(&self) -> &InputLayout {
        &self.input_layout
    }
}
