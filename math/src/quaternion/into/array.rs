use crate::Quaternion;
use std::marker::Destruct;

impl<T> Quaternion<T> {
    /// Convert the elements of this [`Quaternion`] into an array
    pub const fn into_array(self) -> [T; 4]
    where
        T: [const] Destruct,
    {
        [self.x, self.y, self.z, self.w]
    }
}

impl<T: [const] Destruct> const Into<[T; 4]> for Quaternion<T> {
    fn into(self) -> [T; 4] {
        self.into_array()
    }
}
