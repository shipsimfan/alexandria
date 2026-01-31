use crate::math::{Color4, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> Color4<T, Space> {
    /// Combine two colors channel-wise
    pub const fn zip_channels<
        U: [const] Destruct,
        V,
        F: [const] FnMut(T, U) -> V + [const] Destruct,
    >(
        self,
        rhs: Color4<U, Space>,
        mut f: F,
    ) -> Color4<V, Space>
    where
        Space: ColorSpace<U> + ColorSpace<V>,
        T: [const] Destruct,
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
        U: [const] Destruct,
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
    {
        Color4::new(
            f(self.r, rhs.r),
            f(self.g, rhs.g),
            f(self.b, rhs.b),
            f(self.a, rhs.a),
        )
    }

    /// Combine two colors channel-wise with different functions for the RGB channels and the alpha
    /// channel
    pub const fn zip_rgb_a<
        U: [const] Destruct,
        V,
        FC: [const] FnMut(T, U) -> V + [const] Destruct,
        FA: [const] FnOnce(T, U) -> V,
    >(
        self,
        rhs: Color4<U, Space>,
        mut fc: FC,
        fa: FA,
    ) -> Color4<V, Space>
    where
        Space: ColorSpace<U> + ColorSpace<V>,
        T: [const] Destruct,
    {
        Color4::new(
            fc(self.r, rhs.r),
            fc(self.g, rhs.g),
            fc(self.b, rhs.b),
            fa(self.a, rhs.a),
        )
    }

    /// Combine two colors channel-wise and change its space with different functions for the RGB
    /// channels and the alpha channel
    pub const unsafe fn zip_rgb_a_and_space<
        U: [const] Destruct,
        Space2: ColorSpace<U>,
        V,
        Space3: ColorSpace<V>,
        FC: [const] FnMut(T, U) -> V + [const] Destruct,
        FA: [const] FnOnce(T, U) -> V,
    >(
        self,
        rhs: Color4<U, Space2>,
        mut fc: FC,
        fa: FA,
    ) -> Color4<V, Space3>
    where
        T: [const] Destruct,
    {
        Color4::new(
            fc(self.r, rhs.r),
            fc(self.g, rhs.g),
            fc(self.b, rhs.b),
            fa(self.a, rhs.a),
        )
    }
}
