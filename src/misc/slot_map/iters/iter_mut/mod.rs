use crate::misc::slot_map::Slot;

mod iterator;
mod new;

/// An iterator yielding mutable references to the elements of a [`SlotMap`](crate::SlotMap)
pub struct SlotMapIterMut<'a, T> {
    /// The iterator containing the elements
    iter: std::slice::IterMut<'a, Slot<T>>,
}
