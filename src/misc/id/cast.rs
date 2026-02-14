use crate::Id;

impl<T> Id<T> {
    /// Change the type associated with this [`Id`]
    pub const unsafe fn cast<U>(self) -> Id<U> {
        Id::new(self.index(), self.generation)
    }
}
