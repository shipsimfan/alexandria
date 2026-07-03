use crate::math::Rect;
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Create a new [`Rect`] by combining two [`Rect`]s component-wise
    pub const fn zip<
        UP: [const] Destruct,
        US: [const] Destruct,
        P2,
        S2,
        FP: [const] FnMut(P, UP) -> P2 + [const] Destruct,
        FS: [const] FnMut(S, US) -> S2 + [const] Destruct,
    >(
        self,
        rhs: Rect<UP, US>,
        fp: FP,
        fs: FS,
    ) -> Rect<P2, S2>
    where
        P: [const] Destruct,
        S: [const] Destruct,
    {
        Rect::new(
            self.position.zip(rhs.position, fp),
            self.size.zip(rhs.size, fs),
        )
    }

    /// Create a new [`Rect`] by combining three [`Rect`]s component-wise
    pub const fn zip2<
        UP: [const] Destruct,
        VP: [const] Destruct,
        US: [const] Destruct,
        VS: [const] Destruct,
        P2,
        S2,
        FP: [const] FnMut(P, UP, VP) -> P2 + [const] Destruct,
        FS: [const] FnMut(S, US, VS) -> S2 + [const] Destruct,
    >(
        self,
        a: Rect<UP, US>,
        b: Rect<VP, VS>,
        fp: FP,
        fs: FS,
    ) -> Rect<P2, S2>
    where
        P: [const] Destruct,
        S: [const] Destruct,
    {
        Rect::new(
            self.position.zip2(a.position, b.position, fp),
            self.size.zip2(a.size, b.size, fs),
        )
    }

    /// Create a new [`Rect`] by combining four [`Rect`]s component-wise
    pub const fn zip3<
        UP: [const] Destruct,
        VP: [const] Destruct,
        WP: [const] Destruct,
        US: [const] Destruct,
        VS: [const] Destruct,
        WS: [const] Destruct,
        P2,
        S2,
        FP: [const] FnMut(P, UP, VP, WP) -> P2 + [const] Destruct,
        FS: [const] FnMut(S, US, VS, WS) -> S2 + [const] Destruct,
    >(
        self,
        a: Rect<UP, US>,
        b: Rect<VP, VS>,
        c: Rect<WP, WS>,
        fp: FP,
        fs: FS,
    ) -> Rect<P2, S2>
    where
        P: [const] Destruct,
        S: [const] Destruct,
    {
        Rect::new(
            self.position.zip3(a.position, b.position, c.position, fp),
            self.size.zip3(a.size, b.size, c.size, fs),
        )
    }
}
