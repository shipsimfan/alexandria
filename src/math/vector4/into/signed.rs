use crate::math::{Vector4, number::IntoSigned};
use std::marker::Destruct;

const impl<T, U> IntoSigned<Vector4<U>> for Vector4<T>
where
    T: [const] IntoSigned<U> + [const] Destruct,
{
    fn into_signed(self) -> Vector4<U> {
        Vector4::new(
            self.x.into_signed(),
            self.y.into_signed(),
            self.z.into_signed(),
            self.w.into_signed(),
        )
    }
}
