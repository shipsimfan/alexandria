use crate::{
    graphics::color::{Color3, ColorSpace},
    math::number::{Clamp, One, Zero},
};

impl<T: Clamp + Clone + Zero + One, Space: ColorSpace<T>> Color3<T, Space> {
    /// Clamp this color channel wise between 0.0 and 1.0
    pub fn saturate(self) -> Self {
        self.clamp(T::ZERO, T::ONE)
    }
}
