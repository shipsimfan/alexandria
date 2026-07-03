use proc_macro_util::{
    ast::{OuterAttribute, Visibility},
    tokens::{Identifier, Literal},
};
use std::borrow::Cow;

mod parse;

/// Input for the `compile_shader` procedural macro
pub struct CompileShaderInput<'a> {
    /// The attributes to attach to the generated constant
    pub attributes: Vec<OuterAttribute<'a>>,

    /// The visibility to use for the constant
    pub visibility: Option<Visibility<'a>>,

    /// The identifier to use for the constant
    pub identifier: Cow<'a, Identifier>,

    /// The path to the shader source file
    pub path: Cow<'a, Literal>,

    /// The entry point names to compile
    pub entry_points: Vec<Cow<'a, Identifier>>,
}
