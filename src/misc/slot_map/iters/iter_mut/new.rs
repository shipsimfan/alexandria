use crate::{SlotMap, SlotMapIterMut};

impl<'a, T> SlotMapIterMut<'a, T> {
    /// Create an iterator over `slot_map` yielding mutable references to the elements
    pub(in crate::misc::slot_map) fn new(slot_map: &'a mut SlotMap<T>) -> SlotMapIterMut<'a, T> {
        SlotMapIterMut {
            iter: slot_map.slots.iter_mut(),
        }
    }
}
