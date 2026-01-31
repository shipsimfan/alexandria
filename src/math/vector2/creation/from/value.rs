use crate::math::Vector2;

impl<T: [const] Clone> const From<T> for Vector2<T> {
    fn from(value: T) -> Self {
        Vector2::splat(value)
    }
}
