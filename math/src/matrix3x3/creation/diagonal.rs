use crate::{Matrix3x3, Vector3, number::Zero};

impl<T: Zero> Matrix3x3<T> {
    /// Create a new [`Matrix3x3`] with diagonal `(x, y, z)` and all other elements set to 0
    pub const fn diagonal(x: T, y: T, z: T) -> Matrix3x3<T> {
        Matrix3x3 {
            r0: Vector3::new(x, T::ZERO, T::ZERO),
            r1: Vector3::new(T::ZERO, y, T::ZERO),
            r2: Vector3::new(T::ZERO, T::ZERO, z),
        }
    }
}
