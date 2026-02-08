use crate::misc::slot_map::Slot;

impl<T> Slot<T> {
    /// Get the current generation of this slot
    pub fn generation(&self) -> Option<u32> {
        match self {
            Slot::Used { generation, .. } => Some(*generation),
            _ => None,
        }
    }
}
