use crate::program::types::{TypeManager, Vector};

impl Vector {
    /// Add all [`Vector`] types to `types`
    pub(in crate::program::types) fn add_all(types: &mut TypeManager) {
        let f32 = types.get("f32").unwrap().clone();

        types.inner_add(Vector {
            name: "vec2f32",
            size: 2,
            r#type: f32.clone(),
        });
        types.inner_add(Vector {
            name: "vec3f32",
            size: 3,
            r#type: f32.clone(),
        });
        types.inner_add(Vector {
            name: "vec4f32",
            size: 4,
            r#type: f32,
        });
    }
}
