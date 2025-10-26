use crate::{HlslProgram, InputLayout};
use std::borrow::Cow;

impl HlslProgram {
    /// Create a new [`HlslProgram`]
    pub fn new<
        S1: Into<Cow<'static, str>>,
        S2: Into<Cow<'static, str>>,
        S3: Into<Cow<'static, str>>,
    >(
        content: S1,
        input_layout: InputLayout,
        vertex_entry: S2,
        pixel_entry: S3,
    ) -> Self {
        HlslProgram {
            content: content.into(),
            input_layout,
            vertex_entry: vertex_entry.into(),
            pixel_entry: pixel_entry.into(),
        }
    }
}
