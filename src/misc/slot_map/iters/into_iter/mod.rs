use crate::misc::slot_map::Slot;

mod iterator;
mod new;

/// An iterator yielding the elements of a [`SlotMap`](crate::SlotMap)
pub struct SlotMapIntoIter<T> {
    /// The iterator containing the elements
    iter: std::vec::IntoIter<Slot<T>>,
}
