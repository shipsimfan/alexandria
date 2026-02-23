use crate::{
    Error, Result,
    window::{DisplayMode, display::DisplayInner},
};
use std::ptr::null_mut;
use win32::{
    ChangeDisplaySettingsEx, DEVMODE, DISP_CHANGE_SUCCESSFUL, DM_DISPLAYFREQUENCY, DM_PELSHEIGHT,
    DM_PELSWIDTH,
};

impl<UserEvent: Send> DisplayInner<UserEvent> {
    /// Try to set the display mode of this display to the given mode
    pub fn set_fullscreen_mode(&self, mode: DisplayMode) -> Result<()> {
        let mut dev_mode = DEVMODE {
            fields: DM_PELSWIDTH | DM_PELSHEIGHT | DM_DISPLAYFREQUENCY,
            pels_width: mode.size.x,
            pels_height: mode.size.y,
            display_frequency: mode.refresh_rate.as_f32().round() as u32,
            ..Default::default()
        };

        match unsafe {
            ChangeDisplaySettingsEx(
                self.device_name.as_ptr(),
                &mut dev_mode,
                null_mut(),
                0,
                null_mut(),
            )
        } {
            DISP_CHANGE_SUCCESSFUL => Ok(()),
            result => {
                return Err(Error::new(format!(
                    "unable to set fullscreen mode ({})",
                    result
                )));
            }
        }
    }

    /// Try to reset the display mode of this display to the default
    pub fn reset_fullscreen_mode(&self) -> Result<()> {
        match unsafe {
            ChangeDisplaySettingsEx(
                self.device_name.as_ptr(),
                null_mut(),
                null_mut(),
                0,
                null_mut(),
            )
        } {
            DISP_CHANGE_SUCCESSFUL => Ok(()),
            _ => return Err(Error::new("unable to reset fullscreen mode")),
        }
    }
}
