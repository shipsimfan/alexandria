use crate::{
    graphics::color::{Color3, ColorSpace},
    math::number::ApproxEq,
};

impl<T: ApproxEq + Clone, Space: ColorSpace<T>> Color3<T, Space> {
    /// Is this color approximately equal to `other`?
    pub fn approx_eq(self, other: Self, epsilon: T) -> bool {
        self.r.approx_eq(other.r, epsilon.clone())
            && self.g.approx_eq(other.g, epsilon.clone())
            && self.b.approx_eq(other.b, epsilon)
    }
}
