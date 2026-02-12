#[cfg(target_os = "windows")]
use windows::DisplayInner;

#[cfg(target_os = "windows")]
mod windows;

mod new;

/// An active display for presenting images to a user
pub struct Display {
    /// The platform specific implementation of a display
    inner: DisplayInner,
}
