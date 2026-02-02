use crate::MemorySize;

impl const Into<u64> for MemorySize {
    fn into(self) -> u64 {
        self.0
    }
}

impl const Into<usize> for MemorySize {
    fn into(self) -> usize {
        self.0 as _
    }
}
