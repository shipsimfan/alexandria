use crate::{
    graphics::{
        render_context::{SwapchainObjects, SWAP_CHAIN_FLAGS},
        GraphicsContext, RenderContext,
    },
    math::Vector2u,
    Error, Result, BUFFER_COUNT, FORMAT,
};
use win32::try_hresult;

impl RenderContext {
    /// Resize any assets directly tied to window size
    pub(crate) fn resize(
        &mut self,
        graphics_context: &GraphicsContext,
        new_size: Vector2u,
    ) -> Result<()> {
        if self.swapchain_size == new_size {
            return Ok(());
        }

        self.force_resize(graphics_context, new_size)
    }

    /// Forcefully resize any assets directly tied to window size
    pub(in crate::graphics::render_context) fn force_resize(
        &mut self,
        graphics_context: &GraphicsContext,
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
        ))
        .map_err(|os| Error::new_os("unable to resize swapcahin", os))?;

        self.swapchain_objects = Some(SwapchainObjects::new(
            &mut self.swapchain,
            graphics_context.device(),
        )?);

        self.swapchain_size = new_size;
        Ok(())
    }
}
