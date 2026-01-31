use crate::math::Vector4;

impl<T: [const] Default> const Default for Vector4<T> {
    fn default() -> Self {
        Vector4 {
            x: T::default(),
            y: T::default(),
            z: T::default(),
            w: T::default(),
        }
    }
}
