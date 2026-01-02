use crate::MemorySize;

impl MemorySize {
    /// Create a new [`MemorySize`] for `s`
    pub const fn new<T: [const] Into<MemorySize>>(s: T) -> MemorySize {
        s.into()
    }
}
