use crate::compile::ast::items::{Fn, Struct};

mod parse;
mod pretty_print;

/// An item definition
#[derive(Debug)]
pub(in crate::compile) enum Item<'a> {
    /// A function definition
    Fn(Fn<'a>),

    /// A structure defintion
    Struct(Struct<'a>),
}
