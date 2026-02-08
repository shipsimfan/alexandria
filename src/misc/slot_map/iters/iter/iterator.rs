use crate::{SlotMapIter, misc::slot_map::Slot};

impl<'a, T> Iterator for SlotMapIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(slot) = self.iter.next() {
            if let Slot::Used { value, .. } = slot {
                return Some(value);
            }
        }
        None
    }
}
