use crate::Matrix3x3;
use std::marker::Destruct;

impl<T> Matrix3x3<T> {
    /// Cast this [`Matrix3x3`] into another element type
    pub const fn cast<U: [const] From<T>>(self) -> Matrix3x3<U>
    where
        T: [const] Destruct,
    {
        Matrix3x3::new_rows(self.r0.cast(), self.r1.cast(), self.r2.cast())
    }

    /// Cast this [`Matrix3x3`] into another element type
    pub const fn try_cast<
        U: [const] TryFrom<T, Error = Error> + [const] Destruct,
        Error: [const] Destruct,
    >(
        self,
    ) -> Result<Matrix3x3<U>, Error>
    where
        T: [const] Destruct,
    {
        Ok(Matrix3x3::new_rows(
            self.r0.try_cast()?,
            self.r1.try_cast()?,
            self.r2.try_cast()?,
        ))
    }
}
