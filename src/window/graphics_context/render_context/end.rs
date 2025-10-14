use crate::{Error, RenderContext, Result};
use win32::{dxgi::DXGI_PRESENT_ALLOW_TEARING, try_hresult};

impl<'a> RenderContext<'a> {
    /// End the rendering of the current frame
    pub fn end(mut self) -> Result<()> {
        self.frame_ended = true;
        try_hresult!(self.graphics_context.swapchain.present(
            if self.vsync { 1 } else { 0 },
            if self.vsync {
                0
            } else {
                DXGI_PRESENT_ALLOW_TEARING
            }
        ))
        .map_err(|os| Error::new_os("unable to render frame", os))?;
        Ok(())
    }
}
