use crate::{
    Result,
    gpu::{VulkanInstance, VulkanSurface},
    window::Window,
};

impl VulkanSurface {
    /// Create a new [`VulkanSurface`]
    pub(in crate::gpu::instance) fn new<UserEvent: 'static + Send>(
        instance: &VulkanInstance,
        window: &Window<UserEvent>,
    ) -> Result<VulkanSurface> {
        todo!()
    }
}
