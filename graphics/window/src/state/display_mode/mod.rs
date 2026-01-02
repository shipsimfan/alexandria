mod as_str;
mod default;
mod display;

#[cfg(target_os = "windows")]
mod windows;

/// The mode a window should be displayed with
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "data-format",
    derive(data_format::Serialize, data_format::Deserialize)
)]
pub enum DisplayMode {
    /// The window has a thick border and is resizable by it
    Resizable = 0,

    /// The window has a thin border and is not resizable
    Windowed = 1,

    /// The window has no border
    Borderless = 2,
}
