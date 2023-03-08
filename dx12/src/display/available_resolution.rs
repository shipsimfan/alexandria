use crate::{RefreshRate, Resolution, Result, DISPLAY_FORMAT};
use win32::{DXGIOutput1, IDXGIOutput6};

#[derive(PartialEq, Eq)]
pub struct AvailableResolution {
    resolution: Resolution,
    refresh_rates: Vec<RefreshRate>,
}

fn get_resolution<'a>(
    available_resolutions: &'a mut Vec<AvailableResolution>,
    resolution: Resolution,
) -> &'a mut AvailableResolution {
    if let Some(idx) = available_resolutions
        .iter()
        .position(|available_resolution| available_resolution.resolution() == resolution)
    {
        &mut available_resolutions[idx]
    } else {
        available_resolutions.push(AvailableResolution::new(resolution));
        available_resolutions.last_mut().unwrap()
    }
}

impl AvailableResolution {
    pub(crate) fn get(output: &mut IDXGIOutput6) -> Result<Vec<AvailableResolution>> {
        let display_modes = output.get_display_mode_list1(DISPLAY_FORMAT, &[])?;

        let mut available_resolutions = Vec::new();
        for display_mode in display_modes {
            get_resolution(
                &mut available_resolutions,
                Resolution::new(
                    display_mode.width() as usize,
                    display_mode.height() as usize,
                ),
            )
            .add_refresh_rate(RefreshRate::new(
                display_mode.refresh_rate().numerator() as usize,
                display_mode.refresh_rate().denominator() as usize,
            ));
        }

        for available_resolution in &mut available_resolutions {
            available_resolution.sort_refresh_rates();
        }

        available_resolutions.sort_by(|a, b| b.cmp(a));

        Ok(available_resolutions)
    }

    pub(self) fn new(resolution: Resolution) -> Self {
        AvailableResolution {
            resolution,
            refresh_rates: Vec::new(),
        }
    }

    pub fn resolution(&self) -> Resolution {
        self.resolution
    }

    pub fn refresh_rates(&self) -> &[RefreshRate] {
        &self.refresh_rates
    }

    fn add_refresh_rate(&mut self, refresh_rate: RefreshRate) {
        for available_refresh_rate in &self.refresh_rates {
            if *available_refresh_rate == refresh_rate {
                return;
            }
        }

        self.refresh_rates.push(refresh_rate);
    }

    fn sort_refresh_rates(&mut self) {
        self.refresh_rates.sort_by(|a, b| b.cmp(a))
    }
}

impl PartialOrd for AvailableResolution {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AvailableResolution {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.resolution.cmp(&other.resolution)
    }
}
