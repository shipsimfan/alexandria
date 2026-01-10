use crate::DisplayMode;
use alexandria_math::Vector2u;

/// Callbacks for specific window events to allow more efficient processing
pub trait WindowEvents {
    /// Called when the window is resized
    fn on_resize(&mut self, new_size: Vector2u);

    /// Called when the window display mode is changed
    fn on_display_mode_changed(&mut self, new_display_mode: DisplayMode);

    /// Called when a close is requested
    fn on_close_requested(&mut self);

    /// Called when the window either gains or loses focus
    fn on_focus_change(&mut self, is_focused: bool);

    /// Called when the window is maximized
    fn on_maximized(&mut self);

    /// Called when the window is restored from maximized
    fn on_restored(&mut self);
}

impl WindowEvents for () {
    fn on_resize(&mut self, _: Vector2u) {}

    fn on_display_mode_changed(&mut self, _: DisplayMode) {}

    fn on_close_requested(&mut self) {}

    fn on_focus_change(&mut self, _: bool) {}

    fn on_maximized(&mut self) {}

    fn on_restored(&mut self) {}
}
