use crate::D3DProgram;
use std::{borrow::Cow, marker::PhantomData};

impl<'a, Vertex> D3DProgram<'a, Vertex> {
    /// Create a new [`D3DProgram`]
    pub const fn new(vertex_content: Cow<'a, [u8]>, pixel_content: Cow<'a, [u8]>) -> Self {
        D3DProgram {
            vertex_content,
            pixel_content,
            _vertex: PhantomData,
        }
    }
}
