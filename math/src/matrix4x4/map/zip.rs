use crate::Matrix4x4;
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Create a new [`Matrix4x4`] by combining two [`Matrix4x4`]s component-wise
    pub const fn zip<U: [const] Destruct, V, F: [const] FnMut(T, U) -> V + [const] Destruct>(
        self,
        rhs: Matrix4x4<U>,
        mut f: F,
    ) -> Matrix4x4<V>
    where
        T: [const] Destruct,
    {
        Matrix4x4::new_rows(
            self.r0.zip(rhs.r0, &mut f),
            self.r1.zip(rhs.r1, &mut f),
            self.r2.zip(rhs.r2, &mut f),
            self.r3.zip(rhs.r3, f),
        )
    }

    /// Create a new [`Matrix4x4`] by combining three [`Matrix4x4`]s component-wise
    pub const fn zip2<
        U: [const] Destruct,
        V: [const] Destruct,
        W,
        F: [const] FnMut(T, U, V) -> W + [const] Destruct,
    >(
        self,
        a: Matrix4x4<U>,
        b: Matrix4x4<V>,
        mut f: F,
    ) -> Matrix4x4<W>
    where
        T: [const] Destruct,
    {
        Matrix4x4::new_rows(
            self.r0.zip2(a.r0, b.r0, &mut f),
            self.r1.zip2(a.r1, b.r1, &mut f),
            self.r2.zip2(a.r2, b.r2, &mut f),
            self.r3.zip2(a.r3, b.r3, f),
        )
    }

    /// Create a new [`Matrix4x4`] by combining four [`Matrix4x4`]s component-wise
    pub const fn zip3<
        U: [const] Destruct,
        V: [const] Destruct,
        W: [const] Destruct,
        X,
        F: [const] FnMut(T, U, V, W) -> X + [const] Destruct,
    >(
        self,
        a: Matrix4x4<U>,
        b: Matrix4x4<V>,
        c: Matrix4x4<W>,
        mut f: F,
    ) -> Matrix4x4<X>
    where
        T: [const] Destruct,
    {
        Matrix4x4::new_rows(
            self.r0.zip3(a.r0, b.r0, c.r0, &mut f),
            self.r1.zip3(a.r1, b.r1, c.r1, &mut f),
            self.r2.zip3(a.r2, b.r2, c.r2, &mut f),
            self.r3.zip3(a.r3, b.r3, c.r3, f),
        )
    }
}
