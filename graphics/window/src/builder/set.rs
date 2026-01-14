use crate::{DisplayMode, WindowBuilder, WindowEvents};
use alexandria_math::Vector2u;
use std::borrow::Cow;

impl<Callbacks: WindowEvents> WindowBuilder<Callbacks> {
    /// Set the title of the window to be created
    pub fn title<S: Into<Cow<'static, str>>>(mut self, title: S) -> WindowBuilder<Callbacks> {
        self.title = title.into();
        self
    }

    /// Set the window to be created with `size`
    pub fn size(mut self, size: Vector2u) -> WindowBuilder<Callbacks> {
        self.size = Some(size);
        self
    }

    /// Set the window to be created with a default size
    pub fn default_size(mut self) -> WindowBuilder<Callbacks> {
        self.size = None;
        self
    }

    /// Set the display mode to create the window with
    pub fn display_mode(mut self, display_mode: DisplayMode) -> WindowBuilder<Callbacks> {
        self.display_mode = display_mode;
        self
    }

    /// Set the callbacks to be used for window events
    pub fn callbacks<NewCallbacks: WindowEvents>(
        self,
        callbacks: NewCallbacks,
    ) -> WindowBuilder<NewCallbacks> {
        WindowBuilder {
            title: self.title,
            size: self.size,
            display_mode: self.display_mode,
            callbacks,
            #[cfg(target_os = "linux")]
            force_x11: self.force_x11,
        }
    }

    /// Set if X11 should be used over Wayland?
    #[cfg(target_os = "linux")]
    pub fn force_x11(mut self, force_x11: bool) -> WindowBuilder<Callbacks> {
        self.force_x11 = force_x11;
        self
    }
}
