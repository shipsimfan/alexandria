use input::CompileShaderInput;
use proc_macro_util::{
    Token,
    ast::{OuterAttribute, Visibility},
    tokens::{Identifier, Literal},
};
use std::borrow::Cow;

mod input;

mod parse;
mod to_tokens;

/// Compiles a shader at compile time and embeds the resulting SPIR-V binary into the compiled crate
pub struct CompileShader<'a> {
    /// The attributes to attach to the generated constant
    attributes: Vec<OuterAttribute<'a>>,

    /// The visibility to use for the constant
    visibility: Option<Visibility<'a>>,

    /// The name of the constant to generate
    identifier: Cow<'a, Identifier>,

    /// The compiled SPIR-V binary data
    data: Literal,

    /// The length of the compiled SPIR-V binary data
    length: usize,

    /// The entry point names compiled
    entry_points: Vec<(Literal, Token![,])>,
}
