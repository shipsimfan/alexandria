use crate::MemorySize;

const impl From<u64> for MemorySize {
    fn from(value: u64) -> Self {
        MemorySize(value)
    }
}

const impl From<usize> for MemorySize {
    fn from(value: usize) -> Self {
        MemorySize(value as u64)
    }
}
