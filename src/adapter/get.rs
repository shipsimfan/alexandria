use crate::Adapter;

impl Adapter {
    /// Get the name of this adapter
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the dedicated video memory, in bytes, this adapter has
    pub fn video_memory(&self) -> u64 {
        self.video_memory
    }

    /// Is this a software based adapter?
    pub fn is_software(&self) -> bool {
        self.is_software
    }

    /// Is this a hardware based adapter?
    pub fn is_hardware(&self) -> bool {
        !self.is_software
    }
}
