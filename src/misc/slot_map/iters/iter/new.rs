use crate::{SlotMap, SlotMapIter};

impl<'a, T> SlotMapIter<'a, T> {
    /// Create an iterator over `slot_map` yielding references to the elements
    pub(in crate::misc::slot_map) fn new(slot_map: &'a SlotMap<T>) -> SlotMapIter<'a, T> {
        SlotMapIter {
            iter: slot_map.slots.iter(),
        }
    }
}
