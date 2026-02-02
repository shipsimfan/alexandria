use crate::MemorySize;

impl const From<u64> for MemorySize {
    fn from(value: u64) -> Self {
        MemorySize(value)
    }
}

impl const From<usize> for MemorySize {
    fn from(value: usize) -> Self {
        MemorySize(value as u64)
    }
}
