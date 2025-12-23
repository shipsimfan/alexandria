use crate::{Color3, ColorSpace};
use std::marker::PhantomData;

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Create a new [`Color3`]
    pub const fn new(r: T, g: T, b: T) -> Color3<T, Space> {
        Color3 {
            r,
            g,
            b,
            _space: PhantomData,
        }
    }
}
