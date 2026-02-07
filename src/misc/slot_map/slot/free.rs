use crate::{Id, misc::slot_map::Slot};

impl<T> Slot<T> {
    /// Free the element at `id` and return the contained element
    pub fn remove(&mut self, id: Id<T>, next_free: Option<usize>) -> Option<T> {
        match self {
            Slot::Free { .. } => return None,
            Slot::Used {
                generation, value, ..
            } => {
                if id.generation() != *generation {
                    return None;
                }
            }
        }
    }
}
