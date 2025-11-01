use std::{borrow::Cow, marker::PhantomData};

mod get;
mod new;

/// A compiled program ready to be used by Direct 3D
pub struct D3DProgram<'a, Vertex> {
    /// The raw byte content of the vertex shader
    vertex_content: Cow<'a, [u8]>,

    /// The raw byte content of the pixel shader
    pixel_content: Cow<'a, [u8]>,

    /// The vertex type this shader is designed for
    _vertex: PhantomData<Vertex>,
}
