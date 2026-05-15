mod default;
mod set;
mod to_vk;

/// Extended dynamic state features
pub struct VulkanDeviceExtendedDynamicStateFeatures {
    /// Adds additional dynamic state to the pipeline
    pub extended_dynamic_state: bool,
}
