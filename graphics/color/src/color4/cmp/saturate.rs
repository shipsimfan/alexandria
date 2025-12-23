use crate::{Color4, ColorSpace};
use alexandria_math::number::{Clamp, One, Zero};
use std::marker::Destruct;

impl<T: Zero + One, Space: ColorSpace<T>> Color4<T, Space> {
    /// Clamp this color channel wise between normalized 0.0 and 1.0
    pub const fn saturate(self) -> Color4<T, Space>
    where
        T: [const] Clamp + [const] Destruct,
        T::Bound: [const] Clone + [const] Destruct + Zero + One,
        Space: ColorSpace<T::Bound>,
    {
        self.clamp(T::Bound::ZERO, T::Bound::NORMALIZED_ONE)
    }
}
