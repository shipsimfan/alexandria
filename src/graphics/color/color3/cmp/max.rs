use crate::graphics::color::{Color3, ColorSpace};

impl<T: Ord + Clone, Space: ColorSpace<T>> Color3<T, Space> {
    /// Set all channels to be at least `max`
    pub fn max(self, max: T) -> Self {
        self.map_channels(|value| std::cmp::max(value, max.clone()))
    }
}
