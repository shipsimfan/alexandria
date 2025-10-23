//! The definition of types and utilities for types

mod manager;
mod r#type;

mod matrix;
mod primitive;
mod r#struct;
mod vector;

pub use manager::TypeManager;
pub use r#type::Type;

pub use matrix::Matrix;
pub use primitive::Primitive;
pub use r#struct::{Struct, StructField, StructFieldMeta};
pub use vector::Vector;
