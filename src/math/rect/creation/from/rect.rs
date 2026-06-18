use crate::math::Rect;
use std::marker::Destruct;

impl<P, S> Rect<P, S> {
    /// Cast this [`Rect`] into another element type
    pub const fn cast<P2: [const] From<P>, S2: [const] From<S>>(self) -> Rect<P2, S2>
    where
        P: [const] Destruct,
        S: [const] Destruct,
    {
        Rect::new(self.position.cast(), self.size.cast())
    }

    /// Cast this [`Rect`] into another element type
    pub const fn try_cast<
        P2: [const] TryFrom<P, Error = Error> + [const] Destruct,
        S2: [const] TryFrom<S, Error = Error> + [const] Destruct,
        Error: [const] Destruct,
    >(
        self,
    ) -> Result<Rect<P2, S2>, Error>
    where
        P: [const] Destruct,
        S: [const] Destruct,
    {
        Ok(Rect::new(self.position.try_cast()?, self.size.try_cast()?))
    }
}
