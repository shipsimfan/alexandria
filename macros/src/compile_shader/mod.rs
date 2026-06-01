use input::CompileShaderInput;
use proc_macro_util::tokens::Literal;

mod input;

mod parse;
mod to_tokens;

/// Compiles a shader at compile time and embeds the resulting SPIR-V binary into the compiled crate
pub struct CompileShader {
    /// The compiled SPIR-V binary data
    data: Literal,
}
