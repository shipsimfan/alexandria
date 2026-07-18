use crate::math::{Matrix3x3, Vector3, number::FromSigned};
use std::marker::Destruct;

const impl<T, U> FromSigned<Matrix3x3<U>> for Matrix3x3<T>
where
    T: [const] FromSigned<U>,
    U: [const] Destruct,
{
    fn from_signed(value: Matrix3x3<U>) -> Self {
        Matrix3x3::new_rows(
            Vector3::from_signed(value.r0),
            Vector3::from_signed(value.r1),
            Vector3::from_signed(value.r2),
        )
    }
}
