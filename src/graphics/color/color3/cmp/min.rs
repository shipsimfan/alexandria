use crate::graphics::color::{Color3, ColorSpace};

impl<T: Ord + Clone, Space: ColorSpace<T>> Color3<T, Space> {
    /// Set all channels to be at most `min`
    pub fn min(self, min: T) -> Self {
        self.map_channels(|value| std::cmp::min(value, min.clone()))
    }
}
