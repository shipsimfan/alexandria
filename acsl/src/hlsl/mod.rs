use std::borrow::Cow;

mod get;
mod lower;
mod new;

/// A program which has been lowered to HLSL
pub struct HlslProgram {
    /// The HLSL code making up the program
    content: Cow<'static, str>,

    /// The name of the vertex shader entry function
    vertex_entry: Cow<'static, str>,

    /// The name of the pixel shader entry function
    pixel_entry: Cow<'static, str>,
}
