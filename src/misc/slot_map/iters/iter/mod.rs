use crate::misc::slot_map::Slot;

mod iterator;
mod new;

/// An iterator yielding references to the elements of a [`SlotMap`](crate::SlotMap)
pub struct SlotMapIter<'a, T> {
    /// The iterator containing the elements
    iter: std::slice::Iter<'a, Slot<T>>,
}
