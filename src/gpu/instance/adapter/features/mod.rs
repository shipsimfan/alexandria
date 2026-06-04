use vulkan::util::NextChainMut;

mod extended_dynamic_state_features;
mod features;
mod vulkan_1_3_features;

pub use extended_dynamic_state_features::*;
pub use features::*;
pub use vulkan_1_3_features::*;

/// A trait for features that can be queried from a [`VulkanAdapter`] using `get_features`
pub trait VulkanAdapterFeature: NextChainMut {}
