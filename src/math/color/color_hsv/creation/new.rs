use crate::math::{ColorHsv, ColorSpace};
use std::marker::PhantomData;

impl<T, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Create a new [`ColorHsv`]
    pub const fn new(h: T, s: T, v: T) -> ColorHsv<T, Space> {
        ColorHsv {
            h,
            s,
            v,
            _space: PhantomData,
        }
    }
}
