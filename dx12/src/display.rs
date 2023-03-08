use crate::DISPLAY_FORMAT;
use win32::{DXGIOutput1, Interface};

pub struct Display {
    display: win32::IDXGIOutput6,

    name: String,
    display_modes: Vec<DisplayMode>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct DisplayMode {
    width: usize,
    height: usize,
    refresh_rate: RefreshRate,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct RefreshRate {
    inner: win32::DXGIRational,
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
            DisplayMode::new_rational(
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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn display_modes(&self) -> &[DisplayMode] {
        &self.display_modes
    }

    pub fn find_closest_mode(&self, display_mode: &DisplayMode) -> &DisplayMode {
        // Find the modes which are the same or better
        let mut matching_resolution = Vec::new();
        let mut greater_resolution = Vec::new();
        for mode in &self.display_modes {
            if mode.width < display_mode.width {
                // Because of sorting, if the width goes below the target it will never come back above
                break;
            }

            if mode.height < display_mode.height || mode.refresh_rate < display_mode.refresh_rate {
                continue;
            }

            if mode.width == display_mode.width && mode.height == display_mode.height {
                matching_resolution.push(mode);
            } else {
                greater_resolution.push(mode);
            }
        }

        // Find the closest match
        if matching_resolution.len() > 0 {
            let mut best_mode = None;
            let mut lowest_refresh = RefreshRate {
                inner: win32::DXGIRational::new(u32::MAX, 1),
            };

            for mode in matching_resolution {
                if mode.refresh_rate >= display_mode.refresh_rate
                    && display_mode.refresh_rate < lowest_refresh
                {
                    lowest_refresh = mode.refresh_rate;
                    best_mode = Some(mode);
                }
            }

            return best_mode.unwrap();
        }

        // Reversing the list and getting the first with a greater refresh rate
        //   will guarantee that the mode selected has the closest resolution
        greater_resolution.reverse();
        for mode in greater_resolution {
            if mode.refresh_rate >= display_mode.refresh_rate {
                return mode;
            }
        }

        // If no matches, return the best available mode
        &self.display_modes[0]
    }
}

impl DisplayMode {
    pub(crate) fn new_rational(
        width: usize,
        height: usize,
        refresh_rate: win32::DXGIRational,
    ) -> Self {
        DisplayMode {
            width,
            height,
            refresh_rate: RefreshRate {
                inner: refresh_rate,
            },
        }
    }

    pub fn new(width: usize, height: usize, refresh_rate: RefreshRate) -> Self {
        DisplayMode {
            width,
            height,
            refresh_rate,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn refresh_rate(&self) -> RefreshRate {
        self.refresh_rate
    }
}

impl RefreshRate {
    pub fn parse(refresh_rate: &str) -> Option<Self> {
        let mut parts = refresh_rate.split('/');

        let numerator = match parts.next() {
            Some(numerator) => match numerator.parse() {
                Ok(numerator) => numerator,
                Err(_) => return None,
            },
            None => return None,
        };

        let denominator = match parts.next() {
            Some(denominator) => match denominator.parse() {
                Ok(denominator) => denominator,
                Err(_) => return None,
            },
            None => return None,
        };

        match parts.next() {
            Some(_) => None,
            None => Some(RefreshRate {
                inner: win32::DXGIRational::new(numerator, denominator),
            }),
        }
    }

    pub fn serialize(&self) -> String {
        format!("{}/{}", self.inner.numerator(), self.inner.denominator())
    }

    pub fn as_f32(&self) -> f32 {
        self.inner.as_f32()
    }
}
