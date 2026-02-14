use crate::Id;

mod iterator;
mod new;

/// An iterator over a [`PackedMap`](crate::PackedMap) that yields both keys and values
pub struct PackedMapKeyValueIntoIter<T> {
    /// The iterator over the keys
    keys: std::vec::IntoIter<Id<T>>,

    /// The iterator over the values
    values: std::vec::IntoIter<T>,
}
