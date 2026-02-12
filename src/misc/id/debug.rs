use crate::Id;

impl<T> std::fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(std::stringify!(Id))
            .field(std::stringify!(index), &self.index)
            .field(std::stringify!(generation), &self.generation)
            .finish()
    }
}
