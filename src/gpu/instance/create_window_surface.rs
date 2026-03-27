use crate::{
    Result,
    gpu::{VulkanInstance, VulkanSurface},
    window::Window,
};

impl VulkanInstance {
    /// Create a new [`VulkanSurface`] for `window`
    pub fn create_window_surface<UserEvent: 'static + Send>(
        &self,
        window: &Window<UserEvent>,
    ) -> Result<VulkanSurface> {
        VulkanSurface::new(self, window)
    }
}
