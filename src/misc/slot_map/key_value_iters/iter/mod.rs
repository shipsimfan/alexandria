use crate::misc::slot_map::Slot;

mod iterator;
mod new;

/// An iterator over a [`SlotMap`](crate::SlotMap) that yields both keys and references to values
pub struct SlotMapKeyValueIter<'a, T> {
    /// The internal iterator
    iter: std::iter::Enumerate<std::slice::Iter<'a, Slot<T>>>,
}
