use crate::{Matrix3x3, Vector3};

impl<T: [const] Default> const Default for Matrix3x3<T> {
    fn default() -> Self {
        Matrix3x3 {
            r0: Vector3::default(),
            r1: Vector3::default(),
            r2: Vector3::default(),
        }
    }
}
