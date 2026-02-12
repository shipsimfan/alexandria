use crate::Id;

impl<T> Id<T> {
    /// Increase the generation of this ID by 1
    pub(in crate::misc) fn bump_generation(&mut self) {
        self.generation += 1;
    }
}
