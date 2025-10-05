use crate::{GraphicsContext, Result};
use win32::try_hresult;

impl GraphicsContext {
    /// End the rendering for the current frame
    pub(in crate::window) fn end_render(&mut self) -> Result<()> {
        try_hresult!(self.swap_chain.present(if self.vsync { 1 } else { 0 }, 0))?;
        Ok(())
    }
}
