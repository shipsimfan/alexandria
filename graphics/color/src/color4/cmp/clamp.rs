use crate::{Color4, ColorSpace};
use alexandria_math::number::Clamp;
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Clamp this color channel wise between `min` and `max`
    pub const fn clamp(self, min: T, max: T) -> Self
    where
        T: [const] Clamp + [const] Clone + [const] Destruct,
    {
        Color4::new(
            self.r.clamp(min.clone(), max.clone()),
            self.g.clamp(min.clone(), max.clone()),
            self.b.clamp(min.clone(), max.clone()),
            self.a.clamp(min, max),
        )
    }
}
