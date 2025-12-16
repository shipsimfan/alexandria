use crate::graphics::color::{Color4, ColorSpace};

impl<T: std::fmt::Display, Space: ColorSpace<T>> std::fmt::Display for Color4<T, Space> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}
