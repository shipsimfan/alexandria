use crate::misc::slot_map::Slot;

impl<T> Slot<T> {
    /// Set this slot to be used and contain `value`, assuming it is free
    ///
    /// This function returns `(generation, next_free)`
    pub fn set(&mut self, value: T) -> (u32, Option<usize>) {
        let (generation, next_free) = match self {
            Slot::Free {
                next_generation,
                next_free,
            } => (*next_generation, *next_free),
            Slot::Used { .. } => panic!("cannot set a used slot"),
        };

        *self = Slot::Used { generation, value };

        (generation, next_free)
    }
}
