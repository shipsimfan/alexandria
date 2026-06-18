use crate::math::Rect;
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Map the x-position of this [`Rect`] using `f`
    pub const fn map_x<F: [const] FnOnce(P) -> P>(self, f: F) -> Rect<P, S>
    where
        P: [const] Destruct,
        S: [const] Destruct,
    {
        Rect::new(self.position.map_x(f), self.size)
    }

    /// Map the y-position of this [`Rect`] using `f`
    pub const fn map_y<F: [const] FnOnce(P) -> P>(self, f: F) -> Rect<P, S>
    where
        P: [const] Destruct,
        S: [const] Destruct,
    {
        Rect::new(self.position.map_y(f), self.size)
    }

    /// Map the position of this [`Rect`] using `f`
    pub const fn map_position<F: [const] FnMut(P) -> P + [const] Destruct>(self, f: F) -> Rect<P, S>
    where
        P: [const] Destruct,
        S: [const] Destruct,
    {
        Rect::new(self.position.map(f), self.size)
    }

    /// Map the width of this [`Rect`] using `f`
    pub const fn map_width<F: [const] FnOnce(S) -> S>(self, f: F) -> Rect<P, S>
    where
        P: [const] Destruct,
        S: [const] Destruct,
    {
        Rect::new(self.position, self.size.map_x(f))
    }

    /// Map the height of this [`Rect`] using `f`
    pub const fn map_height<F: [const] FnOnce(S) -> S>(self, f: F) -> Rect<P, S>
    where
        P: [const] Destruct,
        S: [const] Destruct,
    {
        Rect::new(self.position, self.size.map_y(f))
    }

    /// Map the size of this [`Rect`] using `f`
    pub const fn map_size<F: [const] FnMut(S) -> S + [const] Destruct>(self, f: F) -> Rect<P, S>
    where
        P: [const] Destruct,
        S: [const] Destruct,
    {
        Rect::new(self.position, self.size.map(f))
    }

    /// Map all the elements of this [`Rect`] component-wise using `f`
    pub const fn map<
        P2,
        S2,
        FP: [const] FnMut(P) -> P2 + [const] Destruct,
        FS: [const] FnMut(S) -> S2 + [const] Destruct,
    >(
        self,
        fp: FP,
        fs: FS,
    ) -> Rect<P2, S2>
    where
        P: [const] Destruct,
        S: [const] Destruct,
    {
        Rect::new(self.position.map(fp), self.size.map(fs))
    }
}
