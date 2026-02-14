use crate::{Error, Result, math::Rect, window::display::DisplayInner};
use win32::{
    GetDpiForMonitor, GetMonitorInfo, HMONITOR, MONITOR_DPI_TYPE, MONITORINFOEX,
    MONITORINFOF_PRIMARY, try_get_last_error, try_hresult,
};

impl DisplayInner {
    /// Create a new [`DisplayInner`]
    pub(in crate::window::display::windows) fn new(handle: HMONITOR) -> Result<DisplayInner> {
        // Get the basic information
        let mut monitor_info = MONITORINFOEX::default();
        try_get_last_error!(GetMonitorInfo(
            handle,
            (&mut monitor_info as *mut MONITORINFOEX).cast()
        ))
        .map_err(|os| Error::new_with("unable to enumerate displays", os))?;

        let rect = Rect::from(monitor_info.monitor);
        let work_area = Rect::from(monitor_info.work);
        let is_primary = monitor_info.flags & MONITORINFOF_PRIMARY != 0;

        // Get the DPI
        let mut dpi_x = 0;
        let mut dpi_y = 0;
        try_hresult!(GetDpiForMonitor(
            handle,
            MONITOR_DPI_TYPE::EffectiveDpi,
            &mut dpi_x,
            &mut dpi_y
        ))
        .map_err(|os| Error::new_with("unable to get display DPI", os))?;
        let dpi = dpi_x;

        // Extract the GDI name
        let mut gdi_name_length = 0;
        for i in 0..monitor_info.device.len() {
            if monitor_info.device[i] == 0 {
                gdi_name_length = i;
                break;
            }
        }

        let gdi_name_utf16 = &monitor_info.device[..gdi_name_length];
        let gdi_name = String::from_utf16_lossy(gdi_name_utf16);

        Ok(DisplayInner {
            rect,
            work_area,
            dpi,
            is_primary,
            name: gdi_name.clone(),
            id: gdi_name,
        })
    }
}
