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

/// An abstract syntax tree describing an ACSL source file
#[derive(Debug)]
pub(in crate::compile) struct Ast<'a> {
    /// The set of items defined in the source file
    items: Vec<Item<'a>>,
}

/// Displays the prefix based on `depth`
fn display_prefix(depth: usize, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for _ in 0..depth {
        write!(f, "  ")?;
    }
    Ok(())
}
