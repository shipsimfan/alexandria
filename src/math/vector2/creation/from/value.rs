use crate::math::Vector2;

const impl<T: [const] Clone> From<T> for Vector2<T> {
    fn from(value: T) -> Self {
        Vector2::splat(value)
    }
}
