use crate::SlotMap;

impl<T> SlotMap<T> {
    /// Get the number of elements in the slot map
    pub const fn len(&self) -> usize {
        self.len
    }

    /// Returns the total number of elements the slot map can hold without reallocating.
    pub const fn capacity(&self) -> usize {
        self.slots.capacity()
    }
}
