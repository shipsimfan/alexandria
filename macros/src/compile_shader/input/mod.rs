use proc_macro_util::tokens::{Identifier, Literal};
use std::borrow::Cow;

mod parse;

/// Input for the `compile_shader` procedural macro
pub struct CompileShaderInput<'a> {
    /// The path to the shader source file
    pub path: Cow<'a, Literal>,

    /// The entry point names to compile
    pub entry_points: Vec<Cow<'a, Identifier>>,
}
