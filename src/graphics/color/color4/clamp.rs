use crate::{
    graphics::color::{Color4, ColorSpace},
    math::number::Clamp,
};

impl<T: Clamp + Clone, Space: ColorSpace<T>> Color4<T, Space> {
    /// Clamp this color channel wise between `min` and `max`
    pub fn clamp(self, min: T, max: T) -> Self {
        self.map_channels(|value| value.clamp(min.clone(), max.clone()))
    }
}
