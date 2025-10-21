use crate::program::types::{Matrix, Primitive, Vector};

mod display;
mod from;
mod id;
mod is_primitive;
mod name;

/// A defined type in the program
#[derive(Debug)]
pub enum Type {
    /// The type is a simple primitive type
    Primitive(Primitive),

    /// A built-in vector primitive types
    Vector(Vector),

    /// A built-in matrix primitive types
    Matrix(Matrix),
}
