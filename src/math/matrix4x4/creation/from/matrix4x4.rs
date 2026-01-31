use crate::math::Matrix4x4;
use std::marker::Destruct;

impl<T> Matrix4x4<T> {
    /// Cast this [`Matrix4x4`] into another element type
    pub const fn cast<U: [const] From<T>>(self) -> Matrix4x4<U>
    where
        T: [const] Destruct,
    {
        Matrix4x4::new_rows(
            self.r0.cast(),
            self.r1.cast(),
            self.r2.cast(),
            self.r3.cast(),
        )
    }

    /// Cast this [`Matrix4x4`] into another element type
    pub const fn try_cast<
        U: [const] TryFrom<T, Error = Error> + [const] Destruct,
        Error: [const] Destruct,
    >(
        self,
    ) -> Result<Matrix4x4<U>, Error>
    where
        T: [const] Destruct,
    {
        Ok(Matrix4x4::new_rows(
            self.r0.try_cast()?,
            self.r1.try_cast()?,
            self.r2.try_cast()?,
            self.r3.try_cast()?,
        ))
    }
}
