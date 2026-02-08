use crate::{SlotMapIterMut, misc::slot_map::Slot};

impl<'a, T> Iterator for SlotMapIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(slot) = self.iter.next() {
            if let Slot::Used { value, .. } = slot {
                return Some(value);
            }
        }
        None
    }
}
