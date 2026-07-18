use crate::math::Vector4;

const impl<T: [const] Clone> From<T> for Vector4<T> {
    fn from(value: T) -> Self {
        Vector4::splat(value)
    }
}
