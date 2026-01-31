use crate::math::Vector3;

impl<T> Vector3<T> {
    /// Create a new [`Vector3`]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Vector3 { x, y, z }
    }
}
