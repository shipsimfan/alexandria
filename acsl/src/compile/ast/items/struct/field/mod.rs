use crate::compile::ast::Attribute;
use lct_diagnostics::Span;

mod get;
mod parse;
mod pretty_print;

/// A field defining a variable in a structure
#[derive(Debug)]
pub(in crate::compile) struct StructField<'a> {
    /// The attributes affecting this field
    attributes: Vec<Attribute<'a>>,

    /// The name of the field
    name: &'a str,

    /// The type of the field
    r#type: &'a str,

    /// The location of this [`StructField`]
    span: Span,
}
