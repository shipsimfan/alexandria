mod add_all;
mod name;

/// A simple primitive type which cannot be deconstructed any further
#[derive(Debug)]
pub enum Primitive {
    /// A 32-bit floating point number
    F32,

    /// A 32-bit unsigned integer
    U32,
}
