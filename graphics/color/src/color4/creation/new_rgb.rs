use crate::{Color4, ColorSpace};
use alexandria_math::number::FromF32;
use std::marker::PhantomData;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Create a new [`Color4`] with a fully opaque alpha channel
    pub const fn new_rgb(r: T, g: T, b: T) -> Color4<T, Space>
    where
        T: [const] FromF32,
    {
        Color4 {
            r,
            g,
            b,
            a: T::from_normalized_f32(1.0),
            _space: PhantomData,
        }
    }
}
