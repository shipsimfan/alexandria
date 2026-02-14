use crate::{SlotMap, SlotMapKeyValueIntoIter};

impl<T> SlotMapKeyValueIntoIter<T> {
    /// Create a new [`SlotMapKeyValueIntoIter`] over `slot_map`
    pub(in crate::misc::slot_map) fn new(slot_map: SlotMap<T>) -> SlotMapKeyValueIntoIter<T> {
        SlotMapKeyValueIntoIter {
            iter: slot_map.slots.into_iter().enumerate(),
        }
    }
}
