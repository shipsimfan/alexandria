use crate::math::Vector2;
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Create a new [`Vector2`] by combining two [`Vector2`]s component-wise
    pub const fn zip<U: [const] Destruct, V, F: [const] FnMut(T, U) -> V + [const] Destruct>(
        self,
        rhs: Vector2<U>,
        mut f: F,
    ) -> Vector2<V>
    where
        T: [const] Destruct,
    {
        Vector2::new(f(self.x, rhs.x), f(self.y, rhs.y))
    }

    /// Create a new [`Vector2`] by combining three [`Vector2`]s component-wise
    pub const fn zip2<
        U: [const] Destruct,
        V: [const] Destruct,
        W,
        F: [const] FnMut(T, U, V) -> W + [const] Destruct,
    >(
        self,
        a: Vector2<U>,
        b: Vector2<V>,
        mut f: F,
    ) -> Vector2<W>
    where
        T: [const] Destruct,
    {
        Vector2::new(f(self.x, a.x, b.x), f(self.y, a.y, b.y))
    }

    /// Create a new [`Vector2`] by combining four [`Vector2`]s component-wise
    pub const fn zip3<
        U: [const] Destruct,
        V: [const] Destruct,
        W: [const] Destruct,
        X,
        F: [const] FnMut(T, U, V, W) -> X + [const] Destruct,
    >(
        self,
        a: Vector2<U>,
        b: Vector2<V>,
        c: Vector2<W>,
        mut f: F,
    ) -> Vector2<X>
    where
        T: [const] Destruct,
    {
        Vector2::new(f(self.x, a.x, b.x, c.x), f(self.y, a.y, b.y, c.y))
    }
}
