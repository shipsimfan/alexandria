use crate::program::types::{Struct, StructField};

impl Struct {
    /// Create a new [`Struct`]
    pub(crate) fn new(name: String, ast_id: u32, fields: Vec<StructField>) -> Self {
        #[cfg(feature = "hlsl")]
        let hlsl_name = format!("__t_{}", name);

        Struct {
            name,
            id: 0,
            ast_id,
            fields,

            #[cfg(feature = "hlsl")]
            hlsl_name,
        }
    }
}
