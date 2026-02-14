use crate::misc::slot_map::Slot;

mod iterator;
mod new;

/// An iterator over a [`SlotMap`](crate::SlotMap) that yields both keys and mutable references to
/// values
pub struct SlotMapKeyValueIterMut<'a, T> {
    /// The internal iterator
    iter: std::iter::Enumerate<std::slice::IterMut<'a, Slot<T>>>,
}
