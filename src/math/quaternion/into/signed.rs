use crate::math::{Quaternion, number::IntoSigned};
use std::marker::Destruct;

impl<T, U> const IntoSigned<Quaternion<U>> for Quaternion<T>
where
    T: [const] IntoSigned<U> + [const] Destruct,
{
    fn into_signed(self) -> Quaternion<U> {
        Quaternion::new(
            self.x.into_signed(),
            self.y.into_signed(),
            self.z.into_signed(),
            self.w.into_signed(),
        )
    }
}
