use crate::math::Rect;
use std::marker::Destruct;

impl<T> Rect<T> {
    /// Create a new [`Rect`] by combining two [`Rect`]s component-wise
    pub const fn zip<U: [const] Destruct, V, F: [const] FnMut(T, U) -> V + [const] Destruct>(
        self,
        rhs: Rect<U>,
        mut f: F,
    ) -> Rect<V>
    where
        T: [const] Destruct,
    {
        Rect::new(
            self.position.zip(rhs.position, &mut f),
            self.size.zip(rhs.size, f),
        )
    }

    /// Create a new [`Rect`] by combining three [`Rect`]s component-wise
    pub const fn zip2<
        U: [const] Destruct,
        V: [const] Destruct,
        W,
        F: [const] FnMut(T, U, V) -> W + [const] Destruct,
    >(
        self,
        a: Rect<U>,
        b: Rect<V>,
        mut f: F,
    ) -> Rect<W>
    where
        T: [const] Destruct,
    {
        Rect::new(
            self.position.zip2(a.position, b.position, &mut f),
            self.size.zip2(a.size, b.size, f),
        )
    }

    /// Create a new [`Rect`] by combining four [`Rect`]s component-wise
    pub const fn zip3<
        U: [const] Destruct,
        V: [const] Destruct,
        W: [const] Destruct,
        X,
        F: [const] FnMut(T, U, V, W) -> X + [const] Destruct,
    >(
        self,
        a: Rect<U>,
        b: Rect<V>,
        c: Rect<W>,
        mut f: F,
    ) -> Rect<X>
    where
        T: [const] Destruct,
    {
        Rect::new(
            self.position
                .zip3(a.position, b.position, c.position, &mut f),
            self.size.zip3(a.size, b.size, c.size, f),
        )
    }
}
