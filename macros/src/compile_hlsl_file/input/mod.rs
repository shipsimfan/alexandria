use proc_macro_util::tokens::{Literal, TokenTree};

mod get;
mod parse;

/// The input to the [`crate::compile_hlsl_file!`] macro
pub struct CompileHlslInput {
    /// The content containing the shader code
    file_name: Literal,

    /// The name of the main vertex function
    vertex_main: Literal,

    /// The name of the main pixel function
    pixel_main: Literal,

    /// The provided input layout
    input_layout: Vec<TokenTree>,
}
