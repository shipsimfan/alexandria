use crate::program::types::{Matrix, Primitive, TypeManager, Vector};

impl TypeManager {
    /// Creates a new [`TypeManager`] with all built-in types
    pub(in crate::program) fn new() -> Self {
        let mut types = TypeManager { types: Vec::new() };

        Primitive::add_all(&mut types);
        Vector::add_all(&mut types);
        Matrix::add_all(&mut types);

        types
    }
}
