use crate::program::types::{Matrix, TypeManager};

impl Matrix {
    /// Add all [`Matrix`] types to `types`
    pub(in crate::program::types) fn add_all(types: &mut TypeManager) {
        types.inner_add(Matrix {
            name: "mat4x4f32",
            columns: 4,
            rows: 4,
            r#type: types.get("f32").unwrap().clone(),
        });
    }
}
