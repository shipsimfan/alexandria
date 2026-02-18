use crate::{Error, Result, window::display::windows::DisplayConfig};
use std::ptr::null_mut;
use win32::{
    DISPLAYCONFIG_PATH_INFO, ERROR_GEN_FAILURE, ERROR_SUCCESS, GetDisplayConfigBufferSizes,
    QDC_ONLY_ACTIVE_PATHS, QueryDisplayConfig,
};

/// Get the active display config paths
fn get_display_config_paths() -> Result<Vec<DISPLAYCONFIG_PATH_INFO>> {
    // Get the counts
    let mut path_count = 0;
    let mut mode_count = 0;
    let result = unsafe {
        GetDisplayConfigBufferSizes(QDC_ONLY_ACTIVE_PATHS, &mut path_count, &mut mode_count)
    };
    if result != ERROR_SUCCESS {
        return Err(Error::new_with(
            "unable to get display configuration count",
            win32::Error::new_win32(result as _),
        ));
    }

    if path_count == 0 || mode_count == 0 {
        return Ok(Vec::new());
    }

    // Get the paths
    let mut paths = Vec::with_capacity(path_count as _);
    let mut modes = Vec::with_capacity(mode_count as _);
    let result = unsafe {
        QueryDisplayConfig(
            QDC_ONLY_ACTIVE_PATHS,
            &mut path_count,
            paths.as_mut_ptr(),
            &mut mode_count,
            modes.as_mut_ptr(),
            null_mut(),
        )
    };
    if result != ERROR_SUCCESS {
        // This can occur when resizing in a virtual environment
        if result == ERROR_GEN_FAILURE {
            return Ok(Vec::new());
        }

        return Err(Error::new_with(
            "unable to get display configurations",
            win32::Error::new_win32(result as _),
        ));
    }

    unsafe { paths.set_len(path_count as _) };

    Ok(paths)
}

impl DisplayConfig {
    /// Enumerate the active [`DisplayConfig`]s
    pub fn enumerate() -> Result<Vec<DisplayConfig>> {
        let paths = get_display_config_paths()?;

        let mut display_configs = Vec::<DisplayConfig>::with_capacity(paths.len());
        'path_loop: for path in paths {
            let display_config = match DisplayConfig::new(&path) {
                Some(display_config) => display_config,
                None => continue,
            };

            // Check for a repeat
            for existing_config in &mut display_configs {
                if existing_config.gdi_name != display_config.gdi_name {
                    continue;
                }

                if (existing_config.id.len() == 0 && display_config.id.len() != 0)
                    || (existing_config.name.len() == 0 && display_config.name.len() != 0)
                {
                    *existing_config = display_config;
                }

                continue 'path_loop;
            }

            display_configs.push(display_config);
        }

        Ok(display_configs)
    }
}
