use crate::{Color3, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color3<T, Space> {
    /// Combine two colors channel-wise
    pub const fn zip_channels<U, V, F: [const] FnMut(T, U) -> V + [const] Destruct>(
        self,
        rhs: Color3<U, Space>,
        mut f: F,
    ) -> Color3<V, Space>
    where
        Space: ColorSpace<U> + ColorSpace<V>,
        T: [const] Destruct,
        U: [const] Destruct,
    {
        Color3::new(f(self.r, rhs.r), f(self.g, rhs.g), f(self.b, rhs.b))
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
        rhs: Color3<U, Space2>,
        mut f: F,
    ) -> Color3<V, Space3>
    where
        T: [const] Destruct,
        U: [const] Destruct,
    {
        Color3::new(f(self.r, rhs.r), f(self.g, rhs.g), f(self.b, rhs.b))
    }
}
