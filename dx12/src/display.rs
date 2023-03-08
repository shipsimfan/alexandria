use crate::DISPLAY_FORMAT;
use win32::{DXGIOutput1, Interface};

pub struct Display {
    display: win32::IDXGIOutput6,

    name: String,
    display_modes: Vec<DisplayMode>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct DisplayMode {
    width: usize,
    height: usize,
    refresh_rate: win32::DXGIRational,
}

fn get_display_name(device_name: &[u16]) -> Result<String, crate::Error> {
    let display_device = win32::enum_display_devices(device_name, 0)?;
    Ok(String::from_utf16(display_device.device_string())
        .unwrap()
        .trim()
        .to_owned())
}

fn get_display_modes(display: &mut win32::IDXGIOutput6) -> Result<Vec<DisplayMode>, crate::Error> {
    let mut modes: Vec<_> = display
        .get_display_mode_list1(DISPLAY_FORMAT, &[])?
        .into_iter()
        .map(|mode| {
            DisplayMode::new(
                mode.width() as usize,
                mode.height() as usize,
                mode.refresh_rate(),
            )
        })
        .collect();

    modes.sort_by(|a, b| b.cmp(a));
    modes.dedup();

    Ok(modes)
}

impl Display {
    pub(crate) fn new(mut display: win32::IDXGIOutput) -> Result<Self, crate::Error> {
        let mut display: win32::IDXGIOutput6 = display.query_interface()?;
        let desc = display.get_desc1()?;

        let name = get_display_name(desc.device_name())?;
        let display_modes = get_display_modes(&mut display)?;

        Ok(Display {
            display,

            name,
            display_modes,
        })
    }
}

impl common::Display for Display {
    type DisplayMode = DisplayMode;
    type DisplayModeIter<'a> = std::slice::Iter<'a, Self::DisplayMode>;

    fn name(&self) -> &str {
        &self.name
    }

    fn display_modes<'a>(&'a self) -> Self::DisplayModeIter<'a> {
        self.display_modes.iter()
    }
}

impl DisplayMode {
    pub(self) fn new(width: usize, height: usize, refresh_rate: win32::DXGIRational) -> Self {
        DisplayMode {
            width,
            height,
            refresh_rate,
        }
    }
}

impl common::DisplayMode for DisplayMode {
    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.height
    }

    fn refresh_rate(&self) -> f32 {
        self.refresh_rate.as_f32()
    }
}
