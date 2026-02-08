mod free;
mod generation;
mod get;
mod set;

/// A slot in a [`SlotMap`](crate::SlotMap)
#[derive(Clone)]
pub(in crate::misc::slot_map) enum Slot<T> {
    /// The slot is not current used
    Free {
        /// The generation to use the next time this slot is filled
        next_generation: u32,

        /// The index of the next free element in the list
        next_free: Option<usize>,
    },

    Used {
        /// The current generation of this slot
        generation: u32,

        /// The contained value
        value: T,
    },
}
