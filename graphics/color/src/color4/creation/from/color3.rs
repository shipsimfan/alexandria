use crate::{Color3, Color4, ColorSpace};
use alexandria_math::number::One;
use std::marker::Destruct;

impl<T: [const] Destruct + One, Space: ColorSpace<T>> const From<Color3<T, Space>>
    for Color4<T, Space>
{
    fn from(color: Color3<T, Space>) -> Self {
        color.with_alpha(T::NORMALIZED_ONE)
    }
}
