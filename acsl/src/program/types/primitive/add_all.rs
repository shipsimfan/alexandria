use crate::program::types::{Primitive, TypeManager};

impl Primitive {
    /// Add all [`Primitive`] types to `types`
    pub(in crate::program::types) fn add_all(types: &mut TypeManager) {
        types.inner_add(Primitive::Unit);
        types.inner_add(Primitive::F32);
        types.inner_add(Primitive::U32);
    }
}
