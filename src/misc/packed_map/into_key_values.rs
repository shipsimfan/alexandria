use crate::{
    PackedMap, PackedMapKeyValueIntoIter, PackedMapKeyValueIter, PackedMapKeyValueIterMut,
};

impl<T> PackedMap<T> {
    /// Convert this map into an iterator over its keys and values
    pub fn into_key_values(self) -> PackedMapKeyValueIntoIter<T> {
        PackedMapKeyValueIntoIter::new(self)
    }

    /// Get at iterator over the keys and references to the values of this map
    pub fn key_value_iter<'a>(&'a self) -> PackedMapKeyValueIter<'a, T> {
        PackedMapKeyValueIter::new(self)
    }

    /// Get at iterator over the keys and mutable references to the values of this map
    pub fn key_value_iter_mut<'a>(&'a mut self) -> PackedMapKeyValueIterMut<'a, T> {
        PackedMapKeyValueIterMut::new(self)
    }
}
