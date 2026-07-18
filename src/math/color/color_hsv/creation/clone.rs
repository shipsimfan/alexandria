use crate::math::{ColorHsv, ColorSpace};

const impl<T: [const] Clone, Space: ColorSpace<T>> Clone for ColorHsv<T, Space> {
    fn clone(&self) -> Self {
        ColorHsv::new(self.h.clone(), self.s.clone(), self.v.clone())
    }
}

impl<T: Copy, Space: ColorSpace<T>> Copy for ColorHsv<T, Space> {}
