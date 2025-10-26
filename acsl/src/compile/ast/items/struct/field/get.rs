use crate::compile::{
    ast::{attribute::Attribute, items::StructField},
    tokens::Identifier,
};
use lct_diagnostics::Span;

impl<'a> StructField<'a> {
    /// Get the attributes affecting this field
    pub const fn attributes(&self) -> &[Attribute<'a>] {
        self.attributes.as_slice()
    }

    /// Get the name of this field
    pub const fn name(&self) -> &'a str {
        self.name
    }

    /// Get the type of this field
    pub const fn r#type(&self) -> &Identifier<'a> {
        &self.r#type
    }

    /// Get the location of this field
    pub const fn span(&self) -> Span {
        self.span
    }
}
