use crate::{Id, misc::slot_map::Slot};

impl<T> Slot<T> {
    /// Free the element at `id` and return the contained element
    pub fn free(&mut self, id: Id<T>, next_free: Option<usize>) -> Option<T> {
        // Verify the ID is valid
        if self.generation() != Some(id.generation()) {
            return None;
        }

        // Swap in a free slot to take the contained element
        let mut new = Slot::Free {
            next_generation: id.generation() + 1,
            next_free,
        };
        std::mem::swap(self, &mut new);
        match new {
            Slot::Used { value, .. } => Some(value),
            _ => unreachable!(),
        }
    }
}
