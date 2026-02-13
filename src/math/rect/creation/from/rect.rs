use crate::math::Rect;
use std::marker::Destruct;

impl<T> Rect<T> {
    /// Cast this [`Rect`] into another element type
    pub const fn cast<U: [const] From<T>>(self) -> Rect<U>
    where
        T: [const] Destruct,
    {
        Rect::new(self.position.cast(), self.size.cast())
    }

    /// Cast this [`Rect`] into another element type
    pub const fn try_cast<
        U: [const] TryFrom<T, Error = Error> + [const] Destruct,
        Error: [const] Destruct,
    >(
        self,
    ) -> Result<Rect<U>, Error>
    where
        T: [const] Destruct,
    {
        Ok(Rect::new(self.position.try_cast()?, self.size.try_cast()?))
    }
}
