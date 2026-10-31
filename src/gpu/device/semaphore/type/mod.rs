mod to_vk;

/// The type of semaphore being created
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VulkanSemaphoreType {
    /// The semaphore is a binary semaphore, which can only be signaled and waited on once
    Binary,

    /// The semaphore is a timeline semaphore, which can be signaled and waited on multiple times
    /// with a monotonically increasing value
    Timeline(u64),
}
