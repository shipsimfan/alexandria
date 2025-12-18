use crate::graphics::color::{Color4, ColorSpace};

impl<T: Ord + Clone, Space: ColorSpace<T>> Color4<T, Space> {
    /// Set all channels to be at least `max`
    pub fn max(self, max: T) -> Self {
        self.map_channels(|value| std::cmp::max(value, max.clone()))
    }
}
