use crate::math::Rect;

impl<P, S> std::fmt::Debug for Rect<P, S>
where
    P: std::fmt::Debug,
    S: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(stringify!(Rect))
            .field("position", &self.position)
            .field("size", &self.size)
            .finish()
    }
}
