use crate::InputLayout;

mod get;
mod lower;

/// A program which has been lowered to HLSL
pub struct HlslProgram {
    /// The HLSL code making up the program
    content: String,

    /// The input layout describing input vertices
    input_layout: InputLayout,

    /// The name of the vertex shader entry function
    vertex_entry: String,

    /// The name of the pixel shader entry function
    pixel_entry: String,
}
