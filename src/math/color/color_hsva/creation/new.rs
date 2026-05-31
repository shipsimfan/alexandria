use crate::math::{ColorHsva, ColorSpace};
use std::marker::PhantomData;

impl<T, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Create a new [`ColorHsva`]
    pub const fn new(h: T, s: T, v: T, a: T) -> ColorHsva<T, Space> {
        ColorHsva {
            h,
            s,
            v,
            a,
            _space: PhantomData,
        }
    }
}
