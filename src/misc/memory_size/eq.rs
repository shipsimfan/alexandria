use crate::MemorySize;

const impl PartialEq for MemorySize {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

const impl PartialEq<u64> for MemorySize {
    fn eq(&self, other: &u64) -> bool {
        self.0.eq(other)
    }
}

const impl PartialEq<usize> for MemorySize {
    fn eq(&self, other: &usize) -> bool {
        self.0.eq(&(*other as u64))
    }
}

const impl Eq for MemorySize {}
