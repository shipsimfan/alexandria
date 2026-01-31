use crate::math::Quaternion;

impl<T> Quaternion<T> {
    /// Get the value at `i`
    pub const fn get(&self, i: usize) -> Option<T>
    where
        T: [const] Clone,
    {
        self.get_ref(i).map(Clone::clone)
    }
}
