use attribute::Attribute;
use expressions::Expression;
use path::Path;
use statements::Statement;

mod attribute;
mod expressions;
mod path;
mod statements;

pub mod items;

mod display;
mod get;
mod parse;
mod pretty_print;

pub(in crate::compile) use items::Item;

/// An abstract syntax tree describing an ACSL source file
#[derive(Debug)]
pub(in crate::compile) struct Ast<'a> {
    /// The set of items defined in the source file
    items: Vec<Item<'a>>,
}
