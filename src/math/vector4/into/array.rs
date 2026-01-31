use crate::math::Vector4;
use std::marker::Destruct;

impl<T> Vector4<T> {
    /// Convert the elements of this [`Vector4`] into an array
    pub const fn into_array(self) -> [T; 4]
    where
        T: [const] Destruct,
    {
        [self.x, self.y, self.z, self.w]
    }
}

impl<T: [const] Destruct> const Into<[T; 4]> for Vector4<T> {
    fn into(self) -> [T; 4] {
        self.into_array()
    }
}
