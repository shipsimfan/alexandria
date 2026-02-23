use crate::{Error, Result, window::display::DisplayInner};
use win32::{GetDpiForMonitor, MONITOR_DPI_TYPE, try_hresult};

impl<UserEvent> DisplayInner<UserEvent> {
    /// Reload the DPI of this display
    pub fn refresh_dpi(&mut self) -> Result<()> {
        let mut dpi_y = 0;
        try_hresult!(GetDpiForMonitor(
            self.handle,
            MONITOR_DPI_TYPE::EffectiveDpi,
            &mut self.dpi,
            &mut dpi_y
        ))
        .map_err(|os| Error::new_with("unable to get display DPI", os))
    }
}
