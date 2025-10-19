mod display;

/// An attribute affecting a node in the AST
#[derive(Debug)]
pub(in crate::compile) struct Attribute<'a> {
    /// The name of the attribute
    name: &'a str,
}
