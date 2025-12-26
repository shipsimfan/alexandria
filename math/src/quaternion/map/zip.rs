use crate::Quaternion;
use std::marker::Destruct;

impl<T> Quaternion<T> {
    /// Create a new [`Quaternion`] by combining two [`Quaternion`]s component-wise
    pub const fn zip<U: [const] Destruct, V, F: [const] FnMut(T, U) -> V + [const] Destruct>(
        self,
        rhs: Quaternion<U>,
        mut f: F,
    ) -> Quaternion<V>
    where
        T: [const] Destruct,
    {
        Quaternion::new(
            f(self.x, rhs.x),
            f(self.y, rhs.y),
            f(self.z, rhs.z),
            f(self.w, rhs.w),
        )
    }

    /// Create a new [`Quaternion`] by combining two [`Quaternion`]s component-wise with different
    /// functions for the x, y, and z-axis and the w-axis
    pub const fn zip_xyz_w<
        U: [const] Destruct,
        V,
        F3: [const] FnMut(T, U) -> V + [const] Destruct,
        FW: [const] FnOnce(T, U) -> V,
    >(
        self,
        rhs: Quaternion<U>,
        mut f3: F3,
        fw: FW,
    ) -> Quaternion<V>
    where
        T: [const] Destruct,
    {
        Quaternion::new(
            f3(self.x, rhs.x),
            f3(self.y, rhs.y),
            f3(self.z, rhs.z),
            fw(self.w, rhs.w),
        )
    }

    /// Create a new [`Quaternion`] by combining three [`Quaternion`]s component-wise
    pub const fn zip2<
        U: [const] Destruct,
        V: [const] Destruct,
        W,
        F: [const] FnMut(T, U, V) -> W + [const] Destruct,
    >(
        self,
        a: Quaternion<U>,
        b: Quaternion<V>,
        mut f: F,
    ) -> Quaternion<W>
    where
        T: [const] Destruct,
    {
        Quaternion::new(
            f(self.x, a.x, b.x),
            f(self.y, a.y, b.y),
            f(self.z, a.z, b.z),
            f(self.w, a.w, b.w),
        )
    }

    /// Create a new [`Quaternion`] by combining three other [`Quaternion`]s component-wise with
    /// different functions for the x, y, and z-axis and the w-axis
    pub const fn zip2_xyz_w<
        U: [const] Destruct,
        V: [const] Destruct,
        W,
        F3: [const] FnMut(T, U, V) -> W + [const] Destruct,
        FW: [const] FnOnce(T, U, V) -> W,
    >(
        self,
        a: Quaternion<U>,
        b: Quaternion<V>,
        mut f3: F3,
        fw: FW,
    ) -> Quaternion<W>
    where
        T: [const] Destruct,
    {
        Quaternion::new(
            f3(self.x, a.x, b.x),
            f3(self.y, a.y, b.y),
            f3(self.z, a.z, b.z),
            fw(self.w, a.w, b.w),
        )
    }

    /// Create a new [`Quaternion`] by combining four [`Quaternion`]s component-wise
    pub const fn zip3<
        U: [const] Destruct,
        V: [const] Destruct,
        W: [const] Destruct,
        X,
        F: [const] FnMut(T, U, V, W) -> X + [const] Destruct,
    >(
        self,
        a: Quaternion<U>,
        b: Quaternion<V>,
        c: Quaternion<W>,
        mut f: F,
    ) -> Quaternion<X>
    where
        T: [const] Destruct,
    {
        Quaternion::new(
            f(self.x, a.x, b.x, c.x),
            f(self.y, a.y, b.y, c.y),
            f(self.z, a.z, b.z, c.z),
            f(self.w, a.w, b.w, c.w),
        )
    }

    /// Create a new [`Quaternion`] by combining four other [`Quaternion`]s component-wise with
    /// different functions for the x, y, and z-axis and the w-axis
    pub const fn zip3_xyz_w<
        U: [const] Destruct,
        V: [const] Destruct,
        W: [const] Destruct,
        X,
        F3: [const] FnMut(T, U, V, W) -> X + [const] Destruct,
        FW: [const] FnOnce(T, U, V, W) -> X,
    >(
        self,
        a: Quaternion<U>,
        b: Quaternion<V>,
        c: Quaternion<W>,
        mut f3: F3,
        fw: FW,
    ) -> Quaternion<X>
    where
        T: [const] Destruct,
    {
        Quaternion::new(
            f3(self.x, a.x, b.x, c.x),
            f3(self.y, a.y, b.y, c.y),
            f3(self.z, a.z, b.z, c.z),
            fw(self.w, a.w, b.w, c.w),
        )
    }
}
