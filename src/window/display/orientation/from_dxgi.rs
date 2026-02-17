use crate::window::DisplayOrientation;
use win32::dxgi::DXGI_MODE_ROTATION;

impl DisplayOrientation {
    /// Convert a DXGI rotation into a [`DisplayOrientation`]
    pub(in crate::window::display) fn from_dxgi(dxgi: DXGI_MODE_ROTATION) -> DisplayOrientation {
        match dxgi {
            DXGI_MODE_ROTATION::Identity => DisplayOrientation::Landscape,
            DXGI_MODE_ROTATION::Rotate90 => DisplayOrientation::Portrait,
            DXGI_MODE_ROTATION::Rotate180 => DisplayOrientation::LandscapeFlipped,
            DXGI_MODE_ROTATION::Rotate270 => DisplayOrientation::PortraitFlipped,
            _ => DisplayOrientation::Landscape,
        }
    }
}
