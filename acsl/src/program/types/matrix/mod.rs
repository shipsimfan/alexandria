use crate::program::Type;
use std::rc::Rc;

mod add_all;
mod display;
mod get;
mod set_id;

/// A built-in matrix of primitive values
#[derive(Debug)]
pub struct Matrix {
    /// The name of this matrix type
    name: &'static str,

    /// The ID given to this type
    id: u32,

    /// The number of columns in matrices of this type
    columns: u8,

    /// The number of rows in matrices of this type
    rows: u8,

    /// The underlying primitive type of the matrix
    r#type: Rc<Type>,
}
