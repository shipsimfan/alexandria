use crate::MemorySize;

impl const Clone for MemorySize {
    fn clone(&self) -> Self {
        MemorySize::new(self.0)
    }
}

impl Copy for MemorySize {}
