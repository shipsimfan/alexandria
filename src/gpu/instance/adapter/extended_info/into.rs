use crate::gpu::{
    VulkanDeviceExtendedDynamicStateFeatures, VulkanDeviceFeatures, VulkanDeviceVulkan13Features,
    VulkanExtendedAdapterInfo,
};

impl VulkanExtendedAdapterInfo {
    /// Get a reference to the inner [`VulkanDeviceFeatures`] if this extended adapter info is of
    /// the correct type
    pub fn as_features(&self) -> Option<&VulkanDeviceFeatures> {
        match self {
            VulkanExtendedAdapterInfo::Features(features) => Some(features),
            _ => None,
        }
    }

    /// Get a mutable reference to the inner [`VulkanDeviceFeatures`] if this extended adapter info
    /// is of the correct type
    pub fn as_features_mut(&mut self) -> Option<&mut VulkanDeviceFeatures> {
        match self {
            VulkanExtendedAdapterInfo::Features(features) => Some(features),
            _ => None,
        }
    }

    /// Attempt to convert this extended adapter info into a [`VulkanDeviceFeatures`] if it is of
    /// the correct type
    pub fn into_features(self) -> Option<VulkanDeviceFeatures> {
        match self {
            VulkanExtendedAdapterInfo::Features(features) => Some(features),
            _ => None,
        }
    }

    /// Get a reference to the inner [`VulkanDeviceVulkan13Features`] if this extended adapter info
    /// is of the correct type
    pub fn as_vulkan_13_features(&self) -> Option<&VulkanDeviceVulkan13Features> {
        match self {
            VulkanExtendedAdapterInfo::Vulkan13Features(features) => Some(features),
            _ => None,
        }
    }

    /// Get a mutable reference to the inner [`VulkanDeviceVulkan13Features`] if this extended
    /// adapter info is of the correct type
    pub fn as_vulkan_13_features_mut(&mut self) -> Option<&mut VulkanDeviceVulkan13Features> {
        match self {
            VulkanExtendedAdapterInfo::Vulkan13Features(features) => Some(features),
            _ => None,
        }
    }

    /// Attempt to convert this extended adapter info into a [`VulkanDeviceVulkan13Features`] if it
    /// is of the correct type
    pub fn into_vulkan_13_features(self) -> Option<VulkanDeviceVulkan13Features> {
        match self {
            VulkanExtendedAdapterInfo::Vulkan13Features(features) => Some(features),
            _ => None,
        }
    }

    /// Get a reference to the inner [`VulkanDeviceExtendedDynamicStateFeatures`] if this extended
    /// adapter info is of the correct type
    pub fn as_extended_dynamic_state_features(
        &self,
    ) -> Option<&VulkanDeviceExtendedDynamicStateFeatures> {
        match self {
            VulkanExtendedAdapterInfo::ExtendedDynamicStateFeatures(features) => Some(features),
            _ => None,
        }
    }

    /// Get a mutable reference to the inner [`VulkanDeviceExtendedDynamicStateFeatures`] if this
    /// extended adapter info is of the correct type
    pub fn as_extended_dynamic_state_features_mut(
        &mut self,
    ) -> Option<&mut VulkanDeviceExtendedDynamicStateFeatures> {
        match self {
            VulkanExtendedAdapterInfo::ExtendedDynamicStateFeatures(features) => Some(features),
            _ => None,
        }
    }

    /// Attempt to convert this extended adapter info into a
    /// [`VulkanDeviceExtendedDynamicStateFeatures`] if it is of the correct type
    pub fn into_extended_dynamic_state_features(
        self,
    ) -> Option<VulkanDeviceExtendedDynamicStateFeatures> {
        match self {
            VulkanExtendedAdapterInfo::ExtendedDynamicStateFeatures(features) => Some(features),
            _ => None,
        }
    }
}
