use crate::MemorySize;
use std::ops::Deref;

impl const Deref for MemorySize {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl const AsRef<u64> for MemorySize {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}
