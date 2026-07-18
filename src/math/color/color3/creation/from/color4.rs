use crate::math::{Color3, Color4, ColorSpace};
use std::marker::Destruct;

const impl<T: [const] Destruct, Space: ColorSpace<T>> From<Color4<T, Space>> for Color3<T, Space> {
    fn from(color: Color4<T, Space>) -> Color3<T, Space> {
        color.rgb()
    }
}
