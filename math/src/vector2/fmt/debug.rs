use crate::Vector2;

impl<T: std::fmt::Debug> std::fmt::Debug for Vector2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Vector2))
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
