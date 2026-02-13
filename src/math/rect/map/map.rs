use crate::math::Rect;
use std::marker::Destruct;

impl<T> Rect<T> {
    /// Map the x-position of this [`Rect`] using `f`
    pub const fn map_x<F: [const] FnOnce(T) -> T>(self, f: F) -> Rect<T>
    where
        T: [const] Destruct,
    {
        Rect::new(self.position.map_x(f), self.size)
    }

    /// Map the y-position of this [`Rect`] using `f`
    pub const fn map_y<F: [const] FnOnce(T) -> T>(self, f: F) -> Rect<T>
    where
        T: [const] Destruct,
    {
        Rect::new(self.position.map_y(f), self.size)
    }

    /// Map the position of this [`Rect`] using `f`
    pub const fn map_position<F: [const] FnMut(T) -> T + [const] Destruct>(self, f: F) -> Rect<T>
    where
        T: [const] Destruct,
    {
        Rect::new(self.position.map(f), self.size)
    }

    /// Map the width of this [`Rect`] using `f`
    pub const fn map_width<F: [const] FnOnce(T) -> T>(self, f: F) -> Rect<T>
    where
        T: [const] Destruct,
    {
        Rect::new(self.position, self.size.map_x(f))
    }

    /// Map the height of this [`Rect`] using `f`
    pub const fn map_height<F: [const] FnOnce(T) -> T>(self, f: F) -> Rect<T>
    where
        T: [const] Destruct,
    {
        Rect::new(self.position, self.size.map_y(f))
    }

    /// Map the size of this [`Rect`] using `f`
    pub const fn map_size<F: [const] FnMut(T) -> T + [const] Destruct>(self, f: F) -> Rect<T>
    where
        T: [const] Destruct,
    {
        Rect::new(self.position, self.size.map(f))
    }

    /// Map all the elements of this [`Rect`] component-wise using `f`
    pub const fn map<U, F: [const] FnMut(T) -> U + [const] Destruct>(self, mut f: F) -> Rect<U>
    where
        T: [const] Destruct,
    {
        Rect::new(self.position.map(&mut f), self.size.map(f))
    }
}
