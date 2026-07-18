use crate::math::Quaternion;

const impl<T: [const] Clone> From<T> for Quaternion<T> {
    fn from(value: T) -> Self {
        Quaternion::splat(value)
    }
}
