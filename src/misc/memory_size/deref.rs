use crate::MemorySize;
use std::ops::Deref;

const impl Deref for MemorySize {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

const impl AsRef<u64> for MemorySize {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}
