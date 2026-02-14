use crate::{Result, window::display::DisplayInner};
use std::ptr::{null, null_mut};
use win32::{BOOL, EnumDisplayMonitors, HDC, HMONITOR, LPARAM, LPRECT, TRUE};

/// Called for each monitor in [`EnumDisplayMonitors`]
unsafe extern "system" fn enumerate_callback(
    handle: HMONITOR,
    _: HDC,
    _: LPRECT,
    l_param: LPARAM,
) -> BOOL {
    let handles = unsafe { &mut *(l_param as *mut Vec<HMONITOR>) };
    handles.push(handle);
    TRUE
}

impl DisplayInner {
    /// Enumerate all currently available displays
    pub fn enumerate() -> Result<Vec<DisplayInner>> {
        let mut handles = Vec::<HMONITOR>::new();
        unsafe {
            EnumDisplayMonitors(
                null_mut(),
                null(),
                enumerate_callback,
                &mut handles as *mut _ as _,
            )
        };

        handles.into_iter().map(DisplayInner::new).collect()
    }
}
