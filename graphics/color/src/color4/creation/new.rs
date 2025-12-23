use crate::{Color4, ColorSpace};
use std::marker::PhantomData;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Create a new [`Color4`]
    pub const fn new(r: T, g: T, b: T, a: T) -> Color4<T, Space> {
        Color4 {
            r,
            g,
            b,
            a,
            _space: PhantomData,
        }
    }
}
