use crate::Vector3;
use std::marker::Destruct;

impl<T> Vector3<T> {
    /// Cast this [`Vector3`] into another element type
    pub const fn cast<U: [const] From<T>>(self) -> Vector3<U>
    where
        T: [const] Destruct,
    {
        Vector3::new(self.x.into(), self.y.into(), self.z.into())
    }

    /// Cast this [`Vector3`] into another element type
    pub const fn try_cast<
        U: [const] TryFrom<T, Error = Error> + [const] Destruct,
        Error: [const] Destruct,
    >(
        self,
    ) -> Result<Vector3<U>, Error>
    where
        T: [const] Destruct,
    {
        Ok(Vector3::new(
            self.x.try_into()?,
            self.y.try_into()?,
            self.z.try_into()?,
        ))
    }
}
