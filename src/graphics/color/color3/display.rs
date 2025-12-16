use crate::graphics::color::{Color3, ColorSpace};

impl<T: std::fmt::Display, Space: ColorSpace<T>> std::fmt::Display for Color3<T, Space> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.r, self.g, self.b)
    }
}
