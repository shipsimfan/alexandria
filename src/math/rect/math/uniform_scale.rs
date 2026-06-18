use crate::math::Rect;
use std::{marker::Destruct, ops::Mul};

impl<P, S> Rect<P, S> {
    /// Scale this [`Rect`] by `scale` uniformally along both axis
    pub const fn uniform_scale(self, scale: S) -> Rect<P, S>
    where
        P: [const] Destruct,
        S: [const] Mul<Output = S> + [const] Clone + [const] Destruct,
    {
        Rect::new(self.position, self.size * scale)
    }
}
