use crate::program::types::{Struct, StructField};

impl Struct {
    /// Create a new [`Struct`]
    pub(crate) const fn new(name: String, ast_id: u32, fields: Vec<StructField>) -> Self {
        Struct {
            name,
            id: 0,
            ast_id,
            fields,
        }
    }
}
