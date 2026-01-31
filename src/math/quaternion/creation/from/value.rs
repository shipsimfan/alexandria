use crate::math::Quaternion;

impl<T: [const] Clone> const From<T> for Quaternion<T> {
    fn from(value: T) -> Self {
        Quaternion::splat(value)
    }
}
