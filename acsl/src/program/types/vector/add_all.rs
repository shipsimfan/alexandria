use crate::program::{
    types::{TypeManager, Vector},
    Type,
};
use std::rc::Rc;

impl Vector {
    /// Create a new [`Vector`]
    fn new(name: &'static str, size: u8, r#type: Rc<Type>) -> Self {
        Vector {
            name,
            id: 0,
            size,
            r#type,
        }
    }

    /// Add all [`Vector`] types to `types`
    pub(in crate::program::types) fn add_all(types: &mut TypeManager) {
        let f32 = types.get("f32").unwrap().clone();

        types.inner_add(Vector::new("vec2f32", 2, f32.clone()));
        types.inner_add(Vector::new("vec3f32", 3, f32.clone()));
        types.inner_add(Vector::new("vec4f32", 4, f32.clone()));
    }
}
