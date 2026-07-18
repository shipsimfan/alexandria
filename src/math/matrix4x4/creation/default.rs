use crate::math::{Matrix4x4, Vector4};

const impl<T: [const] Default> Default for Matrix4x4<T> {
    fn default() -> Self {
        Matrix4x4 {
            r0: Vector4::default(),
            r1: Vector4::default(),
            r2: Vector4::default(),
            r3: Vector4::default(),
        }
    }
}
