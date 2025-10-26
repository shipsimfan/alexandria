mod field;

mod get;
mod new;
mod pretty_print;
mod set_id;

pub use field::{StructField, StructFieldMeta};

/// A structure of multiple heterogenous fields
#[derive(Debug)]
pub struct Struct {
    /// The name of the structure
    name: String,

    /// The ID given to this type
    id: u32,

    /// The ID of the AST node that defined this type
    ast_id: u32,

    /// The fields which make it up
    fields: Vec<StructField>,

    /// The name of this vector type in HLSL
    #[cfg(feature = "hlsl")]
    hlsl_name: String,
}
