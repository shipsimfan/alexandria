use crate::math::{Matrix4x4, number::IntoSigned};
use std::marker::Destruct;

const impl<T, U> IntoSigned<Matrix4x4<U>> for Matrix4x4<T>
where
    T: [const] IntoSigned<U> + [const] Destruct,
{
    fn into_signed(self) -> Matrix4x4<U> {
        Matrix4x4::new_rows(
            self.r0.into_signed(),
            self.r1.into_signed(),
            self.r2.into_signed(),
            self.r3.into_signed(),
        )
    }
}
