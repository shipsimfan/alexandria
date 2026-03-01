use crate::{Error, Result, window::display::DisplayInner};
use win32::{GetDpiForMonitor, MONITOR_DPI_TYPE, try_hresult};

impl<UserEvent> DisplayInner<UserEvent> {
    /// Reload the content scale of this display
    pub fn refresh_content_scale(&mut self) -> Result<()> {
        let mut dpi_x = 0;
        let mut dpi_y = 0;
        try_hresult!(GetDpiForMonitor(
            self.handle,
            MONITOR_DPI_TYPE::EffectiveDpi,
            &mut dpi_x,
            &mut dpi_y
        ))
        .map_err(|os| Error::new_with("unable to get display DPI", os))?;

        self.content_scale = dpi_x as f32 / 96.0;
        Ok(())
    }
}
