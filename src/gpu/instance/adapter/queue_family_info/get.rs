use crate::gpu::VulkanQueueFamilyInfo;

impl VulkanQueueFamilyInfo {
    /// Get the index of this queue family
    ///
    /// This value is used to reference this family in other functions
    pub fn index(&self) -> u32 {
        self.index
    }

    /// Get the number of queues this family contains
    pub fn count(&self) -> u32 {
        self.count
    }

    /// Does this queue support graphics commands?
    pub fn graphics(&self) -> bool {
        self.graphics
    }

    /// Does this queue support compute commands?
    pub fn compute(&self) -> bool {
        self.compute
    }

    /// Does this queue support transfer commands?
    pub fn transfer(&self) -> bool {
        self.transfer
    }
}
