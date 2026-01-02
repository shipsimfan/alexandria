use crate::MemorySize;

impl const PartialEq for MemorySize {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl const PartialEq<u64> for MemorySize {
    fn eq(&self, other: &u64) -> bool {
        self.0.eq(other)
    }
}

impl const PartialEq<usize> for MemorySize {
    fn eq(&self, other: &usize) -> bool {
        self.0.eq(&(*other as u64))
    }
}

impl const Eq for MemorySize {}
