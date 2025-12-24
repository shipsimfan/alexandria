use crate::{Matrix4x4, Vector4};

impl<T: [const] Default> const Default for Matrix4x4<T> {
    fn default() -> Self {
        Matrix4x4 {
            r0: Vector4::default(),
            r1: Vector4::default(),
            r2: Vector4::default(),
            r3: Vector4::default(),
        }
    }
}
