use crate::Vector4;
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Cast this [`Vector4`] into another element type
    pub const fn cast<U: [const] From<T>>(self) -> Vector4<U>
    where
        T: [const] Destruct,
    {
        Vector4::new(self.x.into(), self.y.into(), self.z.into(), self.w.into())
    }

    /// Cast this [`Vector4`] into another element type
    pub const fn try_cast<
        U: [const] TryFrom<T, Error = Error> + [const] Destruct,
        Error: [const] Destruct,
    >(
        self,
    ) -> Result<Vector4<U>, Error>
    where
        T: [const] Destruct,
    {
        Ok(Vector4::new(
            self.x.try_into()?,
            self.y.try_into()?,
            self.z.try_into()?,
            self.w.try_into()?,
        ))
    }
}
