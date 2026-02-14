use crate::{SlotMap, SlotMapKeyValueIterMut};

impl<'a, T> SlotMapKeyValueIterMut<'a, T> {
    /// Create a new [`SlotMapKeyValueIterMut`] over `slot_map`
    pub(in crate::misc::slot_map) fn new(
        slot_map: &'a mut SlotMap<T>,
    ) -> SlotMapKeyValueIterMut<'a, T> {
        SlotMapKeyValueIterMut {
            iter: slot_map.slots.iter_mut().enumerate(),
        }
    }
}
