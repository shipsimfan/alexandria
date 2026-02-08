use crate::{SlotMap, SlotMapIntoIter};

impl<T> SlotMapIntoIter<T> {
    /// Create an iterator over `slot_map` yielding the elements
    pub(in crate::misc::slot_map) fn new(slot_map: SlotMap<T>) -> SlotMapIntoIter<T> {
        SlotMapIntoIter {
            iter: slot_map.slots.into_iter(),
        }
    }
}
