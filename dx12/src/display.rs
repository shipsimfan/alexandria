use win32::Interface;

pub struct Display {
    display: win32::IDXGIOutput6,

    name: String,
}

impl Display {
    pub(crate) fn new(mut display: win32::IDXGIOutput) -> Result<Self, crate::Error> {
        let mut display: win32::IDXGIOutput6 = display.query_interface()?;

        let desc = display.get_desc1()?;

        let display_device = win32::enum_display_devices(desc.device_name(), 0)?;

        let name = format!(
            "{} ({})",
            String::from_utf16(display_device.device_string())
                .unwrap()
                .trim(),
            String::from_utf16(desc.device_name()).unwrap().trim()
        );

        Ok(Display { display, name })
    }
}

impl common::Display for Display {
    fn name(&self) -> &str {
        &self.name
    }
}
