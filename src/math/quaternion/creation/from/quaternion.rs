use crate::math::Quaternion;
use std::marker::Destruct;

impl<T> Quaternion<T> {
    /// Cast this [`Quaternion`] into another element type
    pub const fn cast<U: [const] From<T>>(self) -> Quaternion<U>
    where
        T: [const] Destruct,
    {
        self.map(Into::into)
    }

    /// Cast this [`Quaternion`] into another element type
    pub const fn try_cast<
        U: [const] TryFrom<T, Error = Error> + [const] Destruct,
        Error: [const] Destruct,
    >(
        self,
    ) -> Result<Quaternion<U>, Error>
    where
        T: [const] Destruct,
    {
        Ok(Quaternion::new(
            self.x.try_into()?,
            self.y.try_into()?,
            self.z.try_into()?,
            self.w.try_into()?,
        ))
    }
}
