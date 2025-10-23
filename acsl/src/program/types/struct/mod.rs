mod field;

mod get;
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

    /// The fields which make it up
    fields: Vec<StructField>,
}
