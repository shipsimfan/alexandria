use crate::compile::{
    ast::items::{Struct, StructField},
    tokens::Identifier,
};

impl<'a> Struct<'a> {
    /// Get the name of this struct
    pub const fn name(&self) -> &Identifier<'a> {
        &self.name
    }

    /// Get the ID assigned to this uniquely identify this type definition
    pub const fn id(&self) -> u32 {
        self.type_id
    }

    /// Get the fields that make up this struct
    pub const fn fields(&self) -> &[StructField<'a>] {
        self.fields.as_slice()
    }
}
