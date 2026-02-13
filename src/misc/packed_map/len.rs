use crate::PackedMap;

impl<T> PackedMap<T> {
    /// Get the number of elements in the packed map
    pub const fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns the total number of elements the packed map can hold without reallocating.
    pub const fn capacity(&self) -> usize {
        self.values.capacity()
    }
}
