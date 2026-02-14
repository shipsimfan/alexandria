use crate::misc::slot_map::Slot;

mod iterator;
mod new;

/// An iterator over a [`SlotMap`](crate::SlotMap) that yields both keys and values
pub struct SlotMapKeyValueIntoIter<T> {
    /// The internal iterator
    iter: std::iter::Enumerate<std::vec::IntoIter<Slot<T>>>,
}
