use crate::compile::tokens::Identifier;
use lct_diagnostics::Span;

mod field;

mod get;
mod parse;
mod pretty_print;

pub(in crate::compile) use field::StructField;

/// A structure definition
#[derive(Debug)]
pub(in crate::compile) struct Struct<'a> {
    /// The name of the structure
    name: Identifier<'a>,

    /// The fields making up the struct
    fields: Vec<StructField<'a>>,

    /// The location of the [`Struct`]
    #[allow(unused)]
    span: Span,

    /// An ID that uniquely identifies this AST type definition
    type_id: u32,
}
