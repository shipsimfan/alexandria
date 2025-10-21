use crate::program::{
    types::{Matrix, TypeManager},
    Type,
};
use std::rc::Rc;

impl Matrix {
    /// Create a new [`Matrix`]
    fn new(name: &'static str, columns: u8, rows: u8, r#type: Rc<Type>) -> Self {
        Matrix {
            name,
            id: 0,
            columns,
            rows,
            r#type,
        }
    }

    /// Add all [`Matrix`] types to `types`
    pub(in crate::program::types) fn add_all(types: &mut TypeManager) {
        let f32 = types.get("f32").unwrap().clone();

        types.inner_add(Matrix::new("mat4x4f32", 4, 4, f32));
    }
}
