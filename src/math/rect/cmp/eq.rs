use crate::math::Rect;

const impl<P, S> PartialEq for Rect<P, S>
where
    P: [const] PartialEq,
    S: [const] PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.size == other.size
    }
}

const impl<P, S> Eq for Rect<P, S>
where
    P: [const] Eq,
    S: [const] Eq,
{
}
