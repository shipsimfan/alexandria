use crate::{
    math::{Rational, Vector2u},
    window::DisplayMode,
};
use win32::dxgi::DXGI_MODE_DESC;

impl DisplayMode {
    /// Convert a DXGI mode description into a [`DisplayMode`]
    pub(in crate::window::display) fn from_dxgi(dxgi: &DXGI_MODE_DESC) -> Option<DisplayMode> {
        Some(DisplayMode {
            size: Vector2u::new(dxgi.width, dxgi.height),
            refresh_rate: Rational::from_dxgi(&dxgi.refresh_rate),
        })
    }
}
