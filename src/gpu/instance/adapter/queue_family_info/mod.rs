mod get;
mod new;

/// Information describing a graphics queue family
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VulkanQueueFamilyInfo {
    /// The index of this queue family on the adapter
    index: u32,

    /// The number of queues in the family
    count: u32,

    /// Does this queue support graphics commands?
    graphics: bool,

    /// Does this queue support compute commands?
    compute: bool,

    /// Does this queue support transfer commands?
    transfer: bool,
}
