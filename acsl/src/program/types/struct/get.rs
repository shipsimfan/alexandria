use crate::program::types::{Struct, StructField};

impl Struct {
    /// Gets the name of this struct type
    pub const fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Get the id of this struct type
    pub const fn id(&self) -> u32 {
        self.id
    }

    /// Get the fields that make up this struct
    pub const fn fields(&self) -> &[StructField] {
        self.fields.as_slice()
    }
}
