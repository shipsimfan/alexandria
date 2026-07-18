use crate::math::Vector3;

const impl<T: [const] Clone> From<T> for Vector3<T> {
    fn from(value: T) -> Self {
        Vector3::splat(value)
    }
}
