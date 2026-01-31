use crate::math::Vector3;
use std::marker::Destruct;

impl<T> Vector3<T> {
    /// Convert the elements of this [`Vector3`] into an array
    pub const fn into_array(self) -> [T; 3]
    where
        T: [const] Destruct,
    {
        [self.x, self.y, self.z]
    }
}

impl<T: [const] Destruct> const Into<[T; 3]> for Vector3<T> {
    fn into(self) -> [T; 3] {
        self.into_array()
    }
}
