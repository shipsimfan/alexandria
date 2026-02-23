use crate::{
    Id,
    math::Recti,
    window::{display::DisplayInner, window::WindowInner},
};

impl<UserEvent: 'static + Send> WindowInner<UserEvent> {
    /// Get the position and size of the window's client area, in screen coordinates
    pub fn rect(&self) -> Recti {
        self.window.rect
    }

    /// Is the window currently in fullscreen mode?
    pub fn is_fullscreen(&self) -> bool {
        self.fullscreen_display.is_some()
    }

    /// Get the display that the window is currently fullscreen on, if any
    pub fn fullscreen_display(&self) -> Option<Id<DisplayInner<UserEvent>>> {
        self.fullscreen_display
    }
}
