use crate::Id;

impl<T> Id<T> {
    /// Get the index of the element identified by this ID
    pub(in crate::misc) const fn index(&self) -> usize {
        self.index as _
    }

    /// Get the generation of the element identified by this ID
    pub(in crate::misc) const fn generation(&self) -> u32 {
        self.generation
    }
}
