use crate::math::Rect;

const impl<P, S> Clone for Rect<P, S>
where
    P: [const] Clone,
    S: [const] Clone,
{
    fn clone(&self) -> Self {
        Rect {
            position: self.position.clone(),
            size: self.size.clone(),
        }
    }
}

impl<P, S> Copy for Rect<P, S>
where
    P: Copy,
    S: Copy,
{
}
