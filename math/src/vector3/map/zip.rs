use crate::Vector3;
use std::marker::Destruct;

impl<T> Vector3<T> {
    /// Create a new [`Vector3`] by combining two [`Vector3`]s component-wise
    pub const fn zip<U: [const] Destruct, V, F: [const] FnMut(T, U) -> V + [const] Destruct>(
        self,
        rhs: Vector3<U>,
        mut f: F,
    ) -> Vector3<V>
    where
        T: [const] Destruct,
    {
        Vector3::new(f(self.x, rhs.x), f(self.y, rhs.y), f(self.z, rhs.z))
    }

    /// Create a new [`Vector3`] by combining three [`Vector3`]s component-wise
    pub const fn zip2<
        U: [const] Destruct,
        V: [const] Destruct,
        W,
        F: [const] FnMut(T, U, V) -> W + [const] Destruct,
    >(
        self,
        a: Vector3<U>,
        b: Vector3<V>,
        mut f: F,
    ) -> Vector3<W>
    where
        T: [const] Destruct,
    {
        Vector3::new(
            f(self.x, a.x, b.x),
            f(self.y, a.y, b.y),
            f(self.z, a.z, b.z),
        )
    }

    /// Create a new [`Vector3`] by combining four [`Vector3`]s component-wise
    pub const fn zip3<
        U: [const] Destruct,
        V: [const] Destruct,
        W: [const] Destruct,
        X,
        F: [const] FnMut(T, U, V, W) -> X + [const] Destruct,
    >(
        self,
        a: Vector3<U>,
        b: Vector3<V>,
        c: Vector3<W>,
        mut f: F,
    ) -> Vector3<X>
    where
        T: [const] Destruct,
    {
        Vector3::new(
            f(self.x, a.x, b.x, c.x),
            f(self.y, a.y, b.y, c.y),
            f(self.z, a.z, b.z, c.z),
        )
    }
}
