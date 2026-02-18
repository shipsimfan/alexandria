use crate::{
    Error, Result,
    math::{Rational, Rect, Vector2u},
    window::{
        DisplayMode, DisplayOrientation,
        display::{DisplayInner, windows::DisplayConfig},
    },
};
use std::ptr::{null, null_mut};
use win32::{
    CreateDC, DEVMODE, DISPLAY_DEVICE, DISPLAY_DEVICE_ACTIVE, DISPLAY_DEVICE_MIRRORING_DRIVER,
    DeleteDC, ENUM_CURRENT_SETTINGS, EnumDisplayDevices, EnumDisplaySettingsEx, GetDeviceCaps,
    GetMonitorInfo, HMONITOR, HORZSIZE, MONITORINFOEX, MONITORINFOF_PRIMARY, TRUE, VERTSIZE,
    dxgi::{DXGI_FORMAT, DXGI_OUTPUT_DESC, IDXGIOutput},
    try_get_last_error, try_hresult,
};

const MIN_PHYSICAL_SIZE: i32 = 10;
const MAX_PHYSICAL_SIZE: i32 = 20_000;

/// Finds the null terminator in a UTF-16 string
fn strlen(utf16: &[u16]) -> &[u16] {
    let mut len = 0;
    for i in 0..utf16.len() {
        if utf16[i] == 0 {
            len = i;
            break;
        }
    }

    &utf16[..len]
}

/// Get the DXGI output description of an `output`
fn get_output_desc(output: &mut IDXGIOutput) -> Result<DXGI_OUTPUT_DESC> {
    let mut desc = DXGI_OUTPUT_DESC::default();
    try_hresult!(output.get_desc(&mut desc))
        .map(|_| desc)
        .map_err(|os| Error::new_with("unable to get output description", os))
}

/// Get the basic information describing a monitor
fn get_monitor_info(handle: HMONITOR) -> Result<MONITORINFOEX> {
    let mut monitor_info = MONITORINFOEX::default();
    try_get_last_error!(GetMonitorInfo(
        handle,
        (&mut monitor_info as *mut MONITORINFOEX).cast()
    ))
    .map(|_| monitor_info)
    .map_err(|os| Error::new_with("unable to enumerate displays", os))
}

/// Get the GDI name of a monitor
fn get_gdi_name(output_desc: &DXGI_OUTPUT_DESC) -> (String, &[u16]) {
    let gdi_name_utf16 = strlen(&output_desc.device_name);
    (String::from_utf16_lossy(gdi_name_utf16), gdi_name_utf16)
}

/// Get the phyiscal size of a `monitor`
fn get_physical_size(gdi_name: &[u16]) -> Option<Vector2u> {
    let dc = unsafe { CreateDC(null(), gdi_name.as_ptr(), null(), null()) };
    if dc == null_mut() {
        return None;
    }

    let width = unsafe { GetDeviceCaps(dc, HORZSIZE) };
    let height = unsafe { GetDeviceCaps(dc, VERTSIZE) };
    unsafe { DeleteDC(dc) };

    if width < MIN_PHYSICAL_SIZE
        || height < MIN_PHYSICAL_SIZE
        || width > MAX_PHYSICAL_SIZE
        || height > MAX_PHYSICAL_SIZE
    {
        return None;
    }

    Some(Vector2u::new(width as _, height as _))
}

/// Try to get a stable ID and friendly device name using [`EnumDisplayDevices`]
fn enumerate_display_devices(gdi_name: &[u16]) -> (String, String) {
    let mut adapter = DISPLAY_DEVICE::default();
    let mut i = 0;
    while unsafe { EnumDisplayDevices(null(), i, &mut adapter, 0) } != 0 {
        i += 1;

        // Check GDI name
        let adapter_name = strlen(&adapter.device_name);
        if adapter_name != gdi_name {
            continue;
        }

        // Scan monitors for match
        let mut monitor = DISPLAY_DEVICE::default();
        let mut j = 0;

        let mut best_name: Vec<u16> = Vec::new();
        let mut best_id: Vec<u16> = Vec::new();

        while unsafe { EnumDisplayDevices(adapter.device_name.as_ptr(), j, &mut monitor, 0) } != 0 {
            j += 1;

            if monitor.state_flags & DISPLAY_DEVICE_MIRRORING_DRIVER != 0 {
                continue;
            }

            // Prefer an active monitor
            if monitor.state_flags & DISPLAY_DEVICE_ACTIVE != 0 {
                best_name = strlen(&monitor.device_string).to_vec();
                best_id = strlen(&monitor.device_id).to_vec();
                if best_id.len() == 0 {
                    best_id = strlen(&monitor.device_key).to_vec();
                }

                break;
            }

            // See if this is a fallback candidate
            if best_name.len() == 0 {
                best_name = strlen(&monitor.device_string).to_vec();
                best_id = strlen(&monitor.device_key).to_vec();
            }
        }

        return (
            String::from_utf16_lossy(&best_name),
            String::from_utf16_lossy(&best_id),
        );
    }

    return (String::new(), String::new());
}

/// Get the name and refresh rate of a monitor
fn get_name_and_refresh_rate(
    gdi_name: String,
    gdi_name_utf16: &[u16],
    display_configs: &[DisplayConfig],
) -> (String, String, Rational) {
    let mut matching_display_config = None;
    for display_config in display_configs {
        if display_config.gdi_name() == gdi_name_utf16 {
            matching_display_config = Some(display_config);
            break;
        }
    }

    let (mut name, mut id, refresh_rate) = match matching_display_config {
        Some(display_config) => (
            display_config.name().to_owned(),
            display_config.id().to_owned(),
            display_config.refresh_rate(),
        ),
        None => (String::new(), String::new(), Rational::ZERO),
    };

    // Fallback to [`EnumDisplayDevices`]
    if name.len() == 0 || id.len() == 0 {
        let (enum_name, enum_id) = enumerate_display_devices(gdi_name_utf16);
        if name.len() == 0 && enum_name.len() != 0 {
            name = enum_name;
        }

        if id.len() == 0 && enum_id.len() != 0 {
            id = enum_id;
        }
    }

    // Fallback to GDI name
    if name.len() == 0 {
        name = gdi_name.clone();
    }

    if id.len() == 0 {
        id = gdi_name;
    }

    // Check if the name starts with `\\.\DISPLAY`, and change it if so
    if let Some(number) = name.strip_prefix("\\\\.\\DISPLAY") {
        name = format!("Display {}", number);
    }

    (name, id, refresh_rate)
}

fn get_modes(output: &mut IDXGIOutput) -> Result<Vec<DisplayMode>> {
    let mut num_modes = 0;
    match unsafe {
        output.get_display_mode_list(
            DXGI_FORMAT::B8G8R8A8UNormSRGB,
            0,
            &mut num_modes,
            null_mut(),
        )
    } {
        win32::S_OK => {}
        win32::DXGI_ERROR_NOT_FOUND => return Ok(Vec::new()),
        result => {
            let os = win32::Error::new(result);
            println!("TESTING: {} ({})", os, os.0);
            return Err(Error::new_with("unable to get output mode count", os));
        }
    }
    if num_modes == 0 {
        return Ok(Vec::new());
    }

    let mut modes = Vec::with_capacity(num_modes as _);
    try_hresult!(output.get_display_mode_list(
        DXGI_FORMAT::B8G8R8A8UNormSRGB,
        0,
        &mut num_modes,
        modes.as_mut_ptr()
    ))
    .map_err(|os| Error::new_with("unable to get output modes", os))?;

    unsafe { modes.set_len(num_modes as _) };

    Ok(modes.iter().filter_map(DisplayMode::from_dxgi).collect())
}

/// Get the current refresh rate of the monitor
fn get_refresh_rate(gdi_name: &[u16]) -> Rational {
    let mut dev_mode = DEVMODE::default();
    if unsafe { EnumDisplaySettingsEx(gdi_name.as_ptr(), ENUM_CURRENT_SETTINGS, &mut dev_mode, 0) }
        == 0
    {
        return Rational::ZERO;
    }

    return Rational::from_int(dev_mode.display_frequency as _);
}

impl DisplayInner {
    /// Create a new [`DisplayInner`]
    pub(in crate::window::display::windows) fn new(
        output: &mut IDXGIOutput,
        display_configs: &[DisplayConfig],
    ) -> Result<Option<DisplayInner>> {
        // Get the output description
        let output_desc = get_output_desc(output)?;
        if output_desc.attached_to_desktop != TRUE {
            return Ok(None);
        }

        // Enumerate the modes
        let mut modes = get_modes(output)?;
        modes.sort_by(|a, b| b.cmp(a));

        // Extract info from the output description
        let monitor_info = get_monitor_info(output_desc.monitor)?;

        let (gdi_name, gdi_name_utf16) = get_gdi_name(&output_desc);
        let physical_size = get_physical_size(gdi_name_utf16);

        let orientation = DisplayOrientation::from_dxgi(output_desc.rotation.clone());

        // Extract info from the monitor information
        let rect = Rect::from(monitor_info.monitor);
        let work_area = Rect::from(monitor_info.work);
        let is_primary = monitor_info.flags & MONITORINFOF_PRIMARY != 0;

        // Get the name, ID, and refresh rate
        let (name, id, mut refresh_rate) =
            get_name_and_refresh_rate(gdi_name, gdi_name_utf16, display_configs);
        if refresh_rate == Rational::ZERO {
            refresh_rate = get_refresh_rate(gdi_name_utf16);
        }

        // Create the display structure
        let mut display = DisplayInner {
            handle: output_desc.monitor,
            rect,
            work_area,
            refresh_rate,
            dpi: 0,
            physical_size,
            orientation,
            modes,
            is_primary,
            name,
            id,
        };

        // Get the dpi
        display.refresh_dpi()?;

        Ok(Some(display))
    }
}
