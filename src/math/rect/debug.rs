use crate::math::Rect;

impl<T: std::fmt::Debug> std::fmt::Debug for Rect<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Rect))
            .field("position", &self.position)
            .field("size", &self.size)
            .finish()
    }
}
