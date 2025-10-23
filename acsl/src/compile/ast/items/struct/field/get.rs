use crate::compile::ast::{attribute::Attribute, items::StructField};

impl<'a> StructField<'a> {
    /// Get the attributes affecting this field
    pub const fn attributes(&self) -> &[Attribute] {
        self.attributes.as_slice()
    }
}
