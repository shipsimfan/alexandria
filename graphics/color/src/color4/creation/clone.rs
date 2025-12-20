use crate::{Color4, ColorSpace};

impl<T: [const] Clone, Space: ColorSpace<T>> const Clone for Color4<T, Space> {
    fn clone(&self) -> Self {
        Color4::new(
            self.r.clone(),
            self.g.clone(),
            self.b.clone(),
            self.a.clone(),
        )
    }
}

impl<T: Copy, Space: ColorSpace<T>> Copy for Color4<T, Space> {}
