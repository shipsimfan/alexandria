use crate::math::Vector4;

impl<T> Vector4<T> {
    /// Create a new [`Vector4`]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Vector4 { x, y, z, w }
    }
}
