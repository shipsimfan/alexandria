use crate::MemorySize;

impl MemorySize {
    /// Get the size of `T` as a [`MemorySize`]
    pub const fn size_of<T>() -> MemorySize {
        std::mem::size_of::<T>().into()
    }
}
