use crate::SlotMap;

impl<T> SlotMap<T> {
    /// Create a new [`SlotMap`] without allocating any space
    pub const fn new() -> SlotMap<T> {
        SlotMap {
            slots: Vec::new(),
            len: 0,
            first_free: None,
        }
    }

    /// Create a new [`SlotMap`] with an initial capacity
    pub fn with_capacity(capacity: usize) -> SlotMap<T> {
        SlotMap {
            slots: Vec::with_capacity(capacity),
            len: 0,
            first_free: None,
        }
    }
}
