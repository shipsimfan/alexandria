use crate::{SlotMap, SlotMapKeyValueIntoIter, SlotMapKeyValueIter, SlotMapKeyValueIterMut};

impl<T> SlotMap<T> {
    /// Convert this map into an iterator over its keys and values
    pub fn into_key_values(self) -> SlotMapKeyValueIntoIter<T> {
        SlotMapKeyValueIntoIter::new(self)
    }

    /// Get at iterator over the keys and references to the values of this map
    pub fn key_value_iter<'a>(&'a self) -> SlotMapKeyValueIter<'a, T> {
        SlotMapKeyValueIter::new(self)
    }

    /// Get at iterator over the keys and mutable references to the values of this map
    pub fn key_value_iter_mut<'a>(&'a mut self) -> SlotMapKeyValueIterMut<'a, T> {
        SlotMapKeyValueIterMut::new(self)
    }
}
