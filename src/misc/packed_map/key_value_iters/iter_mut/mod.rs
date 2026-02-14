use crate::Id;

mod iterator;
mod new;

/// An iterator over a [`PackedMap`](crate::PackedMap) that yields both keys and mutable references
/// to values
pub struct PackedMapKeyValueIterMut<'a, T> {
    /// The iterator over the keys
    keys: std::slice::Iter<'a, Id<T>>,

    /// The iterator over the values
    values: std::slice::IterMut<'a, T>,
}
