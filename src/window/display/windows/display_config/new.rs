use crate::{math::Rational, window::display::windows::DisplayConfig};
use std::num::NonZeroU32;
use win32::{
    CCHDEVICENAME, DISPLAYCONFIG_PATH_INFO, DISPLAYCONFIG_SOURCE_DEVICE_NAME,
    DISPLAYCONFIG_TARGET_DEVICE_NAME, DisplayConfigGetDeviceInfo, ERROR_SUCCESS,
};

/// Get the GDI name of the source at `path`
fn get_gdi_name(path: &DISPLAYCONFIG_PATH_INFO) -> Option<[u16; CCHDEVICENAME]> {
    let mut source_name = DISPLAYCONFIG_SOURCE_DEVICE_NAME::default();
    source_name.header.adapter_id = path.source_info.adapter_id;
    source_name.header.id = path.source_info.id;
    let result = unsafe { DisplayConfigGetDeviceInfo(&mut source_name.header) };
    if result != ERROR_SUCCESS {
        return None;
    }

    Some(source_name.view_gdi_device_name)
}

/// Get the name of the target device
fn get_target_name(path: &DISPLAYCONFIG_PATH_INFO) -> Option<DISPLAYCONFIG_TARGET_DEVICE_NAME> {
    let mut target_name = DISPLAYCONFIG_TARGET_DEVICE_NAME::default();
    target_name.header.adapter_id = path.target_info.adapter_id;
    target_name.header.id = path.target_info.id;
    let result = unsafe { DisplayConfigGetDeviceInfo(&mut target_name.header) };
    if result != ERROR_SUCCESS {
        return None;
    }
    Some(target_name)
}

/// Converts an unknown-length UTF-16 string into a [`String`]
fn utf16_to_string(utf16: &[u16]) -> String {
    let mut len = 0;
    for i in 0..utf16.len() {
        if utf16[i] == 0 {
            len = i;
            break;
        }
    }

    String::from_utf16_lossy(&utf16[..len])
}

impl DisplayConfig {
    /// Create a new [`DisplayConfig`] from `path`
    pub(in crate::window::display::windows::display_config) fn new(
        path: &DISPLAYCONFIG_PATH_INFO,
    ) -> Option<DisplayConfig> {
        let gdi_name = get_gdi_name(path)?;
        let mut gdi_name_length = 0;
        for i in 0..gdi_name.len() {
            if gdi_name[i] == 0 {
                gdi_name_length = i;
                break;
            }
        }
        if gdi_name_length == 0 {
            return None;
        }

        let refresh_rate = Rational::new(
            path.target_info.refresh_rate.numerator as _,
            NonZeroU32::new(path.target_info.refresh_rate.denominator).unwrap(),
        );

        let target_name = get_target_name(path)?;

        let name = utf16_to_string(&target_name.monitor_friendly_device_name);
        let id = utf16_to_string(&target_name.monitor_device_path);

        Some(DisplayConfig {
            gdi_name,
            gdi_name_length,
            name,
            id,
            refresh_rate,
        })
    }
}
