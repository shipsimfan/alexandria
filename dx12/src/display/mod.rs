use crate::{instance::Debug, map_instance_error};
use std::sync::{Arc, Mutex};
use win32::Interface;

mod available_resolution;
mod refresh_rate;
mod resolution;

pub use available_resolution::AvailableResolution;
pub use refresh_rate::RefreshRate;
pub use resolution::Resolution;

#[allow(unused)]
pub struct Display {
    display: win32::IDXGIOutput6,

    name: String,
    available_resolutions: Vec<AvailableResolution>,

    position: (isize, isize),
    size: (isize, isize),

    debug: Option<Arc<Mutex<Debug>>>,
}

fn get_display_name(device_name: &[u16]) -> String {
    match win32::enum_display_devices(device_name, 0) {
        Some(display_device) => String::from_utf16(display_device.device_string())
            .unwrap()
            .trim()
            .to_owned(),
        None => String::from("Unnamed"),
    }
}

fn get_position_and_size(monitor: win32::HMonitor) -> ((isize, isize), (isize, isize)) {
    let monitor_rect = match win32::get_monitor_info(monitor) {
        Some(monitor_info) => monitor_info.rc_monitor(),
        None => return ((0, 0), (0, 0)),
    };

    (
        (monitor_rect.left() as isize, monitor_rect.top() as isize),
        (
            monitor_rect.right() as isize - monitor_rect.left() as isize,
            monitor_rect.bottom() as isize - monitor_rect.top() as isize,
        ),
    )
}

impl Display {
    pub(crate) fn new(
        mut display: win32::IDXGIOutput,
        debug: Option<Arc<Mutex<Debug>>>,
    ) -> Result<Self, crate::Error> {
        let mut display: win32::IDXGIOutput6 =
            map_instance_error!(display.query_interface(), CreateDisplay, debug)?;
        let desc = map_instance_error!(display.get_desc1(), CreateDisplay, debug)?;

        let name = get_display_name(desc.device_name());
        let available_resolutions = AvailableResolution::get(&mut display, debug.as_ref())?;

        let (position, size) = get_position_and_size(desc.monitor());

        Ok(Display {
            display,
            name,
            available_resolutions,
            position,
            size,
            debug,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn available_resolutions(&self) -> &[AvailableResolution] {
        &self.available_resolutions
    }

    pub fn find_closest_resolution(
        &self,
        resolution: Option<Resolution>,
        refresh_rate: Option<RefreshRate>,
    ) -> (Resolution, RefreshRate) {
        let resolution = match resolution {
            Some(resolution) => self.closest_resolution(resolution),
            None => self.best_resolution(),
        };

        let refresh_rate = match refresh_rate {
            Some(refresh_rate) => resolution.closest_refresh_rate(refresh_rate),
            None => resolution.best_refresh_rate(),
        };

        (resolution.resolution(), refresh_rate)
    }

    pub(crate) fn position(&self) -> (isize, isize) {
        self.position
    }

    pub(crate) fn size(&self) -> (isize, isize) {
        self.size
    }

    fn closest_resolution(&self, resolution: Resolution) -> &AvailableResolution {
        for available_resolution in self.available_resolutions.iter().rev() {
            if available_resolution.resolution() >= resolution {
                return available_resolution;
            }
        }

        self.best_resolution()
    }

    fn best_resolution(&self) -> &AvailableResolution {
        &self.available_resolutions[0]
    }
}
