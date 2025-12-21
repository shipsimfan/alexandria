use crate::Vector4;
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Create a new [`Vector4`] by combining two other [`Vector4`]s component-wise
    pub const fn zip<U: [const] Destruct, V, F: [const] FnMut(T, U) -> V + [const] Destruct>(
        self,
        rhs: Vector4<U>,
        mut f: F,
    ) -> Vector4<V>
    where
        T: [const] Destruct,
    {
        Vector4::new(
            f(self.x, rhs.x),
            f(self.y, rhs.y),
            f(self.z, rhs.z),
            f(self.w, rhs.w),
        )
    }

    /// Create a new [`Vector4`] by combining two other [`Vector4`]s component-wise with different
    /// functions for the x, y, and z-axis and the w-axis
    pub const fn zip_xyz_w<
        U: [const] Destruct,
        V,
        F3: [const] FnMut(T, U) -> V + [const] Destruct,
        FW: [const] FnOnce(T, U) -> V,
    >(
        self,
        rhs: Vector4<U>,
        mut f3: F3,
        fw: FW,
    ) -> Vector4<V>
    where
        T: [const] Destruct,
    {
        Vector4::new(
            f3(self.x, rhs.x),
            f3(self.y, rhs.y),
            f3(self.z, rhs.z),
            fw(self.w, rhs.w),
        )
    }
}
