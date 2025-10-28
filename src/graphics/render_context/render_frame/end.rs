use crate::{graphics::RenderFrame, Error, Result};
use win32::{dxgi::DXGI_PRESENT_ALLOW_TEARING, try_hresult};

impl<'a> RenderFrame<'a> {
    /// End the rendering of the current frame
    pub fn end(mut self) -> Result<()> {
        self.frame_ended = true;

        try_hresult!(self.render_context.swapchain.present(
            if self.vsync { 1 } else { 0 },
            if self.vsync {
                0
            } else {
                DXGI_PRESENT_ALLOW_TEARING
            }
        ))
        .map_err(|os| Error::new_os("unable to render frame", os))?;

        #[cfg(debug_assertions)]
        self.render_context
            .info_queue
            .empty_queue(self.log_callbacks)?;

        Ok(())
    }
}
