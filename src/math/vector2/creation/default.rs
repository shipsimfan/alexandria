use crate::math::Vector2;

impl<T: [const] Default> const Default for Vector2<T> {
    fn default() -> Self {
        Vector2 {
            x: T::default(),
            y: T::default(),
        }
    }
}
