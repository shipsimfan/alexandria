use crate::math::{Matrix3x3, number::IntoSigned};
use std::marker::Destruct;

const impl<T, U> IntoSigned<Matrix3x3<U>> for Matrix3x3<T>
where
    T: [const] IntoSigned<U> + [const] Destruct,
{
    fn into_signed(self) -> Matrix3x3<U> {
        Matrix3x3::new_rows(
            self.r0.into_signed(),
            self.r1.into_signed(),
            self.r2.into_signed(),
        )
    }
}
