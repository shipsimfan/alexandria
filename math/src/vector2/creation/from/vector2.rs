use crate::Vector2;
use std::marker::Destruct;

impl<T> Vector2<T> {
    /// Cast this [`Vector2`] into another element type
    pub const fn cast<U: [const] From<T>>(self) -> Vector2<U>
    where
        T: [const] Destruct,
    {
        Vector2::new(self.x.into(), self.y.into())
    }

    /// Cast this [`Vector2`] into another element type
    pub const fn try_cast<
        U: [const] TryFrom<T, Error = Error> + [const] Destruct,
        Error: [const] Destruct,
    >(
        self,
    ) -> Result<Vector2<U>, Error>
    where
        T: [const] Destruct,
    {
        Ok(Vector2::new(self.x.try_into()?, self.y.try_into()?))
    }
}
