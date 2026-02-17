#[cfg(target_os = "windows")]
mod from_dxgi;

/// The orientation a display can be in
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DisplayOrientation {
    /// The display is orientated in portait normally
    Portait,

    /// The display is orientated such the right side in portait is on top
    Landscape,

    /// The display is orientated in portait upside down
    PortaitFlipped,

    /// The display is orientated such the left side in portait is on top
    LandscapeFlipped,
}
