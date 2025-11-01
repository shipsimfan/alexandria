use crate::D3DProgram;

impl<'a, Vertex> D3DProgram<'a, Vertex> {
    /// Gets the content of the vertex shader
    pub fn vertex_content(&self) -> &[u8] {
        &self.vertex_content
    }

    /// Gets the content of the pixel shader
    pub fn pixel_content(&self) -> &[u8] {
        &self.pixel_content
    }
}
