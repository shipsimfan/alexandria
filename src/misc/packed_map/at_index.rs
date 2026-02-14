use crate::{Id, PackedMap};

impl<T> PackedMap<T> {
    /// Get the key and a reference to the value at index `i` into the internal array
    ///
    /// These indices will not be stable across inserts or removals, only use it for iteration.
    pub fn at_index(&self, i: usize) -> (Id<T>, &T) {
        (self.ids[i], &self.values[i])
    }

    /// Get the key at index `i` into the internal array
    ///
    /// These indices will not be stable across inserts or removals, only use it for iteration.
    pub fn key_at_index(&self, i: usize) -> Id<T> {
        self.ids[i]
    }

    /// Get a reference to the value at index `i` into the internal array
    ///
    /// These indices will not be stable across inserts or removals, only use it for iteration.
    pub fn value_at_index(&self, i: usize) -> &T {
        &self.values[i]
    }

    /// Get the key and a mutable reference to the value at index `i` into the internal array
    ///
    /// These indices will not be stable across inserts or removals, only use it for iteration.
    pub fn at_index_mut(&mut self, i: usize) -> (Id<T>, &mut T) {
        (self.ids[i], &mut self.values[i])
    }

    /// Get a mutable reference to the value at index `i` into the internal array
    ///
    /// These indices will not be stable across inserts or removals, only use it for iteration.
    pub fn value_at_index_mut(&mut self, i: usize) -> &mut T {
        &mut self.values[i]
    }
}
