use crate::math::Vector3;

impl<T: [const] Clone> const From<T> for Vector3<T> {
    fn from(value: T) -> Self {
        Vector3::splat(value)
    }
}
