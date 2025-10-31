use proc_macro_util::tokens::{Literal, TokenTree};

mod input;

mod parse;
mod to_tokens;

pub struct CompileHlslFile {
    /// The compiled content of the vertex shader
    vertex_content: Literal,

    /// The compiled content of the pixel shader
    pixel_content: Literal,

    /// The provided tokens making up the input layout
    input_layout: Vec<TokenTree>,
}
