use crate::graphics::color::{Color4, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Combine two colors channel-wise
    pub const fn zip_channels<U, V, F: [const] FnMut(T, U) -> V + [const] Destruct>(
        self,
        rhs: Color4<U, Space>,
        mut f: F,
    ) -> Color4<V, Space>
    where
        Space: ColorSpace<U> + ColorSpace<V>,
        T: [const] Destruct,
        U: [const] Destruct,
    {
        Color4::new(
            f(self.r, rhs.r),
            f(self.g, rhs.g),
            f(self.b, rhs.b),
            f(self.a, rhs.a),
        )
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
        rhs: Color4<U, Space2>,
        mut f: F,
    ) -> Color4<V, Space3>
    where
        T: [const] Destruct,
        U: [const] Destruct,
    {
        Color4::new(
            f(self.r, rhs.r),
            f(self.g, rhs.g),
            f(self.b, rhs.b),
            f(self.a, rhs.a),
        )
    }
}
