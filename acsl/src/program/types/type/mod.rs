use crate::program::types::{Matrix, Primitive, Struct, Vector};

mod display;
mod from;
mod id;
mod is_primitive;
mod name;
mod petty_print;

/// A defined type in the program
#[derive(Debug)]
pub enum Type {
    /// The type is a simple primitive type
    Primitive(Primitive),

    /// A built-in vector primitive types
    Vector(Vector),

    /// A built-in matrix primitive types
    Matrix(Matrix),

    /// A user-defined structure
    Struct(Struct),
}
