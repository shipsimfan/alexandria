#[cfg(target_os = "windows")]
mod from_dxgi;

mod rotate;

/// The orientation a display can be in
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisplayOrientation {
    /// The display is orientated in landscape normally
    Landscape,

    /// The display is orientated such the right side in landscape is on top
    Portrait,

    /// The display is orientated in landscape upside down
    LandscapeFlipped,

    /// The display is orientated such the left side in landscape is on top
    PortraitFlipped,
}
