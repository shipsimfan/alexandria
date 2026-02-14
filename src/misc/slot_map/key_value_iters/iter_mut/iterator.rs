use crate::{Id, SlotMapKeyValueIterMut, misc::slot_map::Slot};

impl<'a, T> Iterator for SlotMapKeyValueIterMut<'a, T> {
    type Item = (Id<T>, &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((index, slot)) = self.iter.next() {
            match slot {
                Slot::Used { generation, value } => {
                    return Some((Id::new(index, *generation), value));
                }
                Slot::Free { .. } => {}
            }
        }

        None
    }
}
