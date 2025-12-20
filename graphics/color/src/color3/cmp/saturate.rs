use crate::{Color3, ColorSpace};
use alexandria_math::number::{Clamp, One, Zero};
use std::marker::Destruct;

impl<T: Zero + One, Space: ColorSpace<T>> Color3<T, Space> {
    /// Clamp this color channel wise between normalized 0.0 and 1.0
    pub const fn saturate(self) -> Self
    where
        T: [const] Clamp + [const] Clone + [const] Destruct,
    {
        self.clamp(T::ZERO, T::NORMALIZED_ONE)
    }
}
