use crate::InputLayout;
use std::borrow::Cow;

mod get;
mod new;

/// A compiled program ready to be used by Direct 3D
pub struct D3DProgram<'a> {
    /// The raw byte content of the vertex shader
    vertex_content: Cow<'a, [u8]>,

    /// The raw byte content of the pixel shader
    pixel_content: Cow<'a, [u8]>,

    /// The input layout describing input vertices
    input_layout: InputLayout,
}
