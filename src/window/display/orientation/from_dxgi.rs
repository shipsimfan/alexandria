use crate::window::DisplayOrientation;
use win32::dxgi::DXGI_MODE_ROTATION;

impl DisplayOrientation {
    /// Convert a DXGI rotation into a [`DisplayOrientation`]
    pub(in crate::window::display) fn from_dxgi(dxgi: DXGI_MODE_ROTATION) -> DisplayOrientation {
        match dxgi {
            DXGI_MODE_ROTATION::Identity => DisplayOrientation::Portait,
            DXGI_MODE_ROTATION::Rotate90 => DisplayOrientation::Landscape,
            DXGI_MODE_ROTATION::Rotate180 => DisplayOrientation::PortaitFlipped,
            DXGI_MODE_ROTATION::Rotate270 => DisplayOrientation::LandscapeFlipped,
            _ => DisplayOrientation::Portait,
        }
    }
}
