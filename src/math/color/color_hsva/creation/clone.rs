use crate::math::{ColorHsva, ColorSpace};

impl<T: [const] Clone, Space: ColorSpace<T>> const Clone for ColorHsva<T, Space> {
    fn clone(&self) -> Self {
        ColorHsva::new(
            self.h.clone(),
            self.s.clone(),
            self.v.clone(),
            self.a.clone(),
        )
    }
}

impl<T: Copy, Space: ColorSpace<T>> Copy for ColorHsva<T, Space> {}
