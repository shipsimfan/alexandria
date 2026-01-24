use crate::{
    GraphicsDeviceExtendedCreateInfo, GraphicsDeviceExtendedDynamicStateFeatures,
    GraphicsDeviceFeatures, GraphicsDeviceVulkan13Features,
};

impl From<GraphicsDeviceFeatures> for GraphicsDeviceExtendedCreateInfo {
    fn from(features: GraphicsDeviceFeatures) -> Self {
        GraphicsDeviceExtendedCreateInfo::Features(features)
    }
}

impl From<GraphicsDeviceVulkan13Features> for GraphicsDeviceExtendedCreateInfo {
    fn from(features: GraphicsDeviceVulkan13Features) -> Self {
        GraphicsDeviceExtendedCreateInfo::Vulkan13Features(features)
    }
}

impl From<GraphicsDeviceExtendedDynamicStateFeatures> for GraphicsDeviceExtendedCreateInfo {
    fn from(features: GraphicsDeviceExtendedDynamicStateFeatures) -> Self {
        GraphicsDeviceExtendedCreateInfo::ExtendedDynamicStateFeatures(features)
    }
}
