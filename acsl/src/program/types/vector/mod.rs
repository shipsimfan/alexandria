use crate::program::Type;
use std::rc::Rc;

mod add_all;
mod get;
mod pretty_print;
mod set_id;

/// A built-in vector of primitive values
#[derive(Debug)]
pub struct Vector {
    /// The name of this vector type
    name: &'static str,

    /// The ID given to this type
    id: u32,

    /// The number of elements in vectors of this type
    size: u8,

    /// The underlying primitive type of the vector
    r#type: Rc<Type>,

    /// The name of this vector type in HLSL
    #[cfg(feature = "hlsl")]
    hlsl_name: &'static str,
}
