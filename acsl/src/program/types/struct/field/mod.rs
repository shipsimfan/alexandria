use crate::program::Type;
use std::rc::Rc;

mod meta;

mod get;
mod pretty_print;

pub use meta::StructFieldMeta;

/// A field in a structure
#[derive(Debug)]
pub struct StructField {
    /// The metadata flags affecting this field
    meta: Vec<StructFieldMeta>,

    /// The name of the field
    name: String,

    /// The type of the field
    r#type: Rc<Type>,
}
