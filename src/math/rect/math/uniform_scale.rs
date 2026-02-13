use crate::math::Rect;
use std::{marker::Destruct, ops::Mul};

impl<T> Rect<T> {
    /// Scale this [`Rect`] by `scale` uniformally along both axis
    pub const fn uniform_scale(self, scale: T) -> Rect<T>
    where
        T: [const] Mul<Output = T> + [const] Clone + [const] Destruct,
    {
        Rect::new(self.position, self.size * scale)
    }
}
