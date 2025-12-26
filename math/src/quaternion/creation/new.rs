use crate::Quaternion;

impl<T> Quaternion<T> {
    /// Create a new [`Quaternion`]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Quaternion { x, y, z, w }
    }
}
