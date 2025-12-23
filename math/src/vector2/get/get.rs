use crate::Vector2;

impl<T> Vector2<T> {
    /// Get the value at `i`
    pub const fn get(&self, i: usize) -> Option<T>
    where
        T: [const] Clone,
    {
        self.get_ref(i).map(Clone::clone)
    }
}
