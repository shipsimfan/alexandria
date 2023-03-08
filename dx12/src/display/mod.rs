use win32::Interface;

mod available_resolution;
mod refresh_rate;
mod resolution;

pub use available_resolution::AvailableResolution;
pub use refresh_rate::RefreshRate;
pub use resolution::Resolution;

pub struct Display {
    display: win32::IDXGIOutput6,

    name: String,
    available_resolutions: Vec<AvailableResolution>,
}

fn get_display_name(device_name: &[u16]) -> Result<String, crate::Error> {
    let display_device = win32::enum_display_devices(device_name, 0)?;
    Ok(String::from_utf16(display_device.device_string())
        .unwrap()
        .trim()
        .to_owned())
}

impl Display {
    pub(crate) fn new(mut display: win32::IDXGIOutput) -> Result<Self, crate::Error> {
        let mut display: win32::IDXGIOutput6 = display.query_interface()?;
        let desc = display.get_desc1()?;

        let name = get_display_name(desc.device_name())?;
        let available_resolutions = AvailableResolution::get(&mut display)?;

        Ok(Display {
            display,
            name,
            available_resolutions,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn available_resolutions(&self) -> &[AvailableResolution] {
        &self.available_resolutions
    }
}
