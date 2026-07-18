use crate::math::Vector2;

const impl<T: [const] Default> Default for Vector2<T> {
    fn default() -> Self {
        Vector2 {
            x: T::default(),
            y: T::default(),
        }
    }
}
