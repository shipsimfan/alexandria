use crate::{GraphicsContext, Result};
use win32::{dxgi::DXGI_PRESENT_ALLOW_TEARING, try_hresult};

impl GraphicsContext {
    /// End the rendering for the current frame
    pub(in crate::window) fn end_render(&mut self, vsync: bool) -> Result<()> {
        try_hresult!(self.swap_chain.present(
            if vsync { 1 } else { 0 },
            if vsync { 0 } else { DXGI_PRESENT_ALLOW_TEARING }
        ))?;
        Ok(())
    }
}
