use crate::Vector4;

impl<T: [const] Clone> const From<T> for Vector4<T> {
    fn from(value: T) -> Self {
        Vector4::splat(value)
    }
}
