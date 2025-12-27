use crate::Matrix3x3;
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Create a new [`Matrix3x3`] by combining two [`Matrix3x3`]s component-wise
    pub const fn zip<U: [const] Destruct, V, F: [const] FnMut(T, U) -> V + [const] Destruct>(
        self,
        rhs: Matrix3x3<U>,
        mut f: F,
    ) -> Matrix3x3<V>
    where
        T: [const] Destruct,
    {
        Matrix3x3::new_rows(
            self.r0.zip(rhs.r0, &mut f),
            self.r1.zip(rhs.r1, &mut f),
            self.r2.zip(rhs.r2, &mut f),
        )
    }

    /// Create a new [`Matrix3x3`] by combining three [`Matrix3x3`]s component-wise
    pub const fn zip2<
        U: [const] Destruct,
        V: [const] Destruct,
        W,
        F: [const] FnMut(T, U, V) -> W + [const] Destruct,
    >(
        self,
        a: Matrix3x3<U>,
        b: Matrix3x3<V>,
        mut f: F,
    ) -> Matrix3x3<W>
    where
        T: [const] Destruct,
    {
        Matrix3x3::new_rows(
            self.r0.zip2(a.r0, b.r0, &mut f),
            self.r1.zip2(a.r1, b.r1, &mut f),
            self.r2.zip2(a.r2, b.r2, &mut f),
        )
    }

    /// Create a new [`Matrix3x3`] by combining four [`Matrix3x3`]s component-wise
    pub const fn zip3<
        U: [const] Destruct,
        V: [const] Destruct,
        W: [const] Destruct,
        X,
        F: [const] FnMut(T, U, V, W) -> X + [const] Destruct,
    >(
        self,
        a: Matrix3x3<U>,
        b: Matrix3x3<V>,
        c: Matrix3x3<W>,
        mut f: F,
    ) -> Matrix3x3<X>
    where
        T: [const] Destruct,
    {
        Matrix3x3::new_rows(
            self.r0.zip3(a.r0, b.r0, c.r0, &mut f),
            self.r1.zip3(a.r1, b.r1, c.r1, &mut f),
            self.r2.zip3(a.r2, b.r2, c.r2, &mut f),
        )
    }
}
