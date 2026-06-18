use crate::math::{Quaternion, number::FromSigned};

impl<T, U> FromSigned<Quaternion<U>> for Quaternion<T>
where
    T: FromSigned<U>,
{
    fn from_signed(value: Quaternion<U>) -> Self {
        Quaternion::new(
            T::from_signed(value.x),
            T::from_signed(value.y),
            T::from_signed(value.z),
            T::from_signed(value.w),
        )
    }
}
