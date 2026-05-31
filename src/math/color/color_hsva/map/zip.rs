use crate::math::{ColorHsva, ColorSpace};
use std::marker::Destruct;

impl<T, Space: ColorSpace<T>> ColorHsva<T, Space> {
    /// Combine two colors channel-wise
    pub const fn zip_channels<U, V, F: [const] FnMut(T, U) -> V + [const] Destruct>(
        self,
        rhs: ColorHsva<U, Space>,
        mut f: F,
    ) -> ColorHsva<V, Space>
    where
        Space: ColorSpace<U> + ColorSpace<V>,
        T: [const] Destruct,
        U: [const] Destruct,
    {
        ColorHsva::new(
            f(self.h, rhs.h),
            f(self.s, rhs.s),
            f(self.v, rhs.v),
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
        rhs: ColorHsva<U, Space2>,
        mut f: F,
    ) -> ColorHsva<V, Space3>
    where
        T: [const] Destruct,
        U: [const] Destruct,
    {
        ColorHsva::new(
            f(self.h, rhs.h),
            f(self.s, rhs.s),
            f(self.v, rhs.v),
            f(self.a, rhs.a),
        )
    }

    /// Combine two colors channel-wise with different functions for the HSV channels and the alpha
    /// channel
    pub const fn zip_hsv_a<
        U,
        V,
        FH: [const] FnMut(T, U) -> V + [const] Destruct,
        FA: [const] FnOnce(T, U) -> V,
    >(
        self,
        rhs: ColorHsva<U, Space>,
        mut fh: FH,
        fa: FA,
    ) -> ColorHsva<V, Space>
    where
        Space: ColorSpace<U> + ColorSpace<V>,
        T: [const] Destruct,
        U: [const] Destruct,
    {
        ColorHsva::new(
            fh(self.h, rhs.h),
            fh(self.s, rhs.s),
            fh(self.v, rhs.v),
            fa(self.a, rhs.a),
        )
    }

    /// Combine two colors channel-wise with different functions for the HSV channels and the alpha
    /// channel and change its space
    pub const fn zip_hsv_a_and_space<
        U,
        Space2: ColorSpace<U>,
        V,
        Space3: ColorSpace<V>,
        FH: [const] FnMut(T, U) -> V + [const] Destruct,
        FA: [const] FnOnce(T, U) -> V,
    >(
        self,
        rhs: ColorHsva<U, Space2>,
        mut fh: FH,
        fa: FA,
    ) -> ColorHsva<V, Space3>
    where
        T: [const] Destruct,
        U: [const] Destruct,
    {
        ColorHsva::new(
            fh(self.h, rhs.h),
            fh(self.s, rhs.s),
            fh(self.v, rhs.v),
            fa(self.a, rhs.a),
        )
    }
}
