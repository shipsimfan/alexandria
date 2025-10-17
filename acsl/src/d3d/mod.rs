use crate::InputLayout;

/// A compiled program ready to be used by Direct 3D
pub struct D3DProgram {
    /// The raw byte content of the program
    content: Vec<u8>,

    /// The input layout describing input vertices
    input_layout: InputLayout,

    /// The name of the vertex shader entry function
    vertex_entry: String,

    /// The name of the pixel shader entry function
    pixel_entry: String,
}
