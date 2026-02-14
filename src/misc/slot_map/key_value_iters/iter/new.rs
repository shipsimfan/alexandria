use crate::{SlotMap, SlotMapKeyValueIter};

impl<'a, T> SlotMapKeyValueIter<'a, T> {
    /// Create a new [`SlotMapKeyValueIter`] over `slot_map`
    pub(in crate::misc::slot_map) fn new(slot_map: &'a SlotMap<T>) -> SlotMapKeyValueIter<'a, T> {
        SlotMapKeyValueIter {
            iter: slot_map.slots.iter().enumerate(),
        }
    }
}
