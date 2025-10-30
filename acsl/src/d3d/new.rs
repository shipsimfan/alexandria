use crate::{D3DProgram, InputLayout};
use std::borrow::Cow;

impl<'a> D3DProgram<'a> {
    /// Create a new [`D3DProgram`]
    pub const fn new(
        vertex_content: Cow<'a, [u8]>,
        pixel_content: Cow<'a, [u8]>,
        input_layout: InputLayout,
    ) -> Self {
        D3DProgram {
            vertex_content,
            pixel_content,
            input_layout,
        }
    }
}
