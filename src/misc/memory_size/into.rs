use crate::MemorySize;

const impl Into<u64> for MemorySize {
    fn into(self) -> u64 {
        self.0
    }
}

const impl Into<usize> for MemorySize {
    fn into(self) -> usize {
        self.0 as _
    }
}
