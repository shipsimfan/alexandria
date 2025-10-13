use crate::{
    math::Vector2u,
    window::graphics_context::{SwapchainObjects, SWAP_CHAIN_FLAGS},
    GraphicsContext, Result, BUFFER_COUNT, FORMAT,
};
use win32::try_hresult;

impl GraphicsContext {
    /// Resize any assets directly tied to window size
    pub(in crate::window) fn resize(&mut self, new_size: Vector2u) -> Result<()> {
        if self.swapchain_size == new_size {
            return Ok(());
        }

        self.force_resize(new_size)
    }

    /// Forcefully resize any assets directly tied to window size
    pub(in crate::window::graphics_context) fn force_resize(
        &mut self,
        new_size: Vector2u,
    ) -> Result<()> {
        if let Some(swapchain_objects) = &mut self.swapchain_objects {
            swapchain_objects.unbind(&mut self.device_context);
        }
        self.swapchain_objects = None;

        try_hresult!(self.swapchain.resize_buffers(
            BUFFER_COUNT,
            new_size.x,
            new_size.y,
            FORMAT,
            SWAP_CHAIN_FLAGS,
        ))?;

        self.swapchain_objects = Some(SwapchainObjects::new(&mut self.swapchain, &self.device)?);

        self.swapchain_size = new_size;
        Ok(())
    }
}
