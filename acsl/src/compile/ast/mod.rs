use attribute::Attribute;
use expressions::Expression;
use items::Item;
use path::Path;
use statements::Statement;

mod attribute;
mod expressions;
mod items;
mod path;
mod statements;

mod display;
mod parse;
mod pretty_print;

/// An abstract syntax tree describing an ACSL source file
#[derive(Debug)]
pub(in crate::compile) struct Ast<'a> {
    /// The set of items defined in the source file
    items: Vec<Item<'a>>,
}
