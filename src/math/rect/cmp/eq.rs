use crate::math::Rect;

impl<P, S> const PartialEq for Rect<P, S>
where
    P: [const] PartialEq,
    S: [const] PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.size == other.size
    }
}

impl<P, S> const Eq for Rect<P, S>
where
    P: [const] Eq,
    S: [const] Eq,
{
}
