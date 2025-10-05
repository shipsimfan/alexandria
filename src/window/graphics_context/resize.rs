use crate::{math::Vector2u, GraphicsContext, Result, BUFFER_COUNT, FORMAT};
use win32::try_hresult;

impl GraphicsContext {
    /// Resize any assets directly tied to window size
    pub(in crate::window) fn resize(&mut self, new_size: Vector2u) -> Result<()> {
        try_hresult!(self.swap_chain.resize_buffers(
            BUFFER_COUNT,
            new_size.x,
            new_size.y,
            FORMAT,
            0
        ))?;
        Ok(())
    }
}
