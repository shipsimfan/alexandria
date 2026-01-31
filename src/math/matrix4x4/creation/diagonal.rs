use crate::math::{Matrix4x4, Vector4, number::Zero};

impl<T: Zero> Matrix4x4<T> {
    /// Create a new [`Matrix4x4`] with diagonal `(x, y, z, w)` and all other elements set to 0
    pub const fn diagonal(x: T, y: T, z: T, w: T) -> Matrix4x4<T> {
        Matrix4x4 {
            r0: Vector4::new(x, T::ZERO, T::ZERO, T::ZERO),
            r1: Vector4::new(T::ZERO, y, T::ZERO, T::ZERO),
            r2: Vector4::new(T::ZERO, T::ZERO, z, T::ZERO),
            r3: Vector4::new(T::ZERO, T::ZERO, T::ZERO, w),
        }
    }
}
