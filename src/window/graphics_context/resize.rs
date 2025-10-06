use crate::{
    math::Vector2u, window::graphics_context::SWAP_CHAIN_FLAGS, GraphicsContext, Result,
    BUFFER_COUNT, FORMAT,
};
use win32::try_hresult;

impl GraphicsContext {
    /// Resize any assets directly tied to window size
    pub(in crate::window) fn resize(&mut self, new_size: Vector2u) -> Result<()> {
        if self.swap_chain_size == new_size {
            return Ok(());
        }

        try_hresult!(self.swap_chain.resize_buffers(
            BUFFER_COUNT,
            new_size.x,
            new_size.y,
            FORMAT,
            SWAP_CHAIN_FLAGS,
        ))?;

        self.swap_chain_size = new_size;
        Ok(())
    }
}
