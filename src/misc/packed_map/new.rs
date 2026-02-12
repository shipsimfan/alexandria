use crate::PackedMap;

impl<T> PackedMap<T> {
    /// Create a new [`PackedMap`] without allocating any space
    pub const fn new() -> PackedMap<T> {
        PackedMap {
            values: Vec::new(),
            ids: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// Create a new [`PackedMap`] with an initial capacity
    pub fn with_capacity(capacity: usize) -> PackedMap<T> {
        PackedMap {
            values: Vec::with_capacity(capacity),
            ids: Vec::with_capacity(capacity),
            indices: Vec::with_capacity(capacity),
        }
    }
}
