use crate::{Color3, ColorSpace};

impl<T: [const] Clone, Space: ColorSpace<T>> const Clone for Color3<T, Space> {
    fn clone(&self) -> Self {
        Color3::new(self.r.clone(), self.g.clone(), self.b.clone())
    }
}

impl<T: Copy, Space: ColorSpace<T>> Copy for Color3<T, Space> {}
