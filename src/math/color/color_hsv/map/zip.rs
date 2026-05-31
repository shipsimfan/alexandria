use crate::math::{ColorHsv, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsv<T, Space> {
    /// Combine two colors channel-wise
    pub const fn zip_channels<U, V, F: [const] FnMut(T, U) -> V + [const] Destruct>(
        self,
        rhs: ColorHsv<U, Space>,
        mut f: F,
    ) -> ColorHsv<V, Space>
    where
        Space: ColorSpace<U> + ColorSpace<V>,
        T: [const] Destruct,
        U: [const] Destruct,
    {
        ColorHsv::new(f(self.h, rhs.h), f(self.s, rhs.s), f(self.v, rhs.v))
    }

    /// Combine two colors channel-wise and change its space
    pub const unsafe fn zip_channels_and_space<
        U,
        Space2: ColorSpace<U>,
        V,
        Space3: ColorSpace<V>,
        F: [const] FnMut(T, U) -> V + [const] Destruct,
    >(
        self,
        rhs: ColorHsv<U, Space2>,
        mut f: F,
    ) -> ColorHsv<V, Space3>
    where
        T: [const] Destruct,
        U: [const] Destruct,
    {
        ColorHsv::new(f(self.h, rhs.h), f(self.s, rhs.s), f(self.v, rhs.v))
    }
}
