use crate::math::{Vector3, number::IntoSigned};
use std::marker::Destruct;

impl<T, U> const IntoSigned<Vector3<U>> for Vector3<T>
where
    T: [const] IntoSigned<U> + [const] Destruct,
{
    fn into_signed(self) -> Vector3<U> {
        Vector3::new(
            self.x.into_signed(),
            self.y.into_signed(),
            self.z.into_signed(),
        )
    }
}
