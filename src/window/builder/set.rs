use crate::{Adapter, DisplayMode, WindowBuilder};

impl<'a> WindowBuilder<'a> {
    /// Set the window's title
    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    /// Set the x position of the window
    pub fn x(mut self, x: i32) -> Self {
        self.x = Some(x);
        self
    }

    /// Set the y position of the window
    pub fn y(mut self, y: i32) -> Self {
        self.y = Some(y);
        self
    }

    /// Set the width of the window
    pub fn width(mut self, width: u32) -> Self {
        self.width = Some(width);
        self
    }

    /// Set the height of the window
    pub fn height(mut self, height: u32) -> Self {
        self.height = Some(height);
        self
    }

    /// Set the mode the window should be displayed in
    pub fn display_mode(mut self, display_mode: DisplayMode) -> Self {
        self.display_mode = display_mode;
        self
    }

    /// Set the window to [`DisplayMode::Resizable`]
    pub fn resizable(self) -> Self {
        self.display_mode(DisplayMode::Resizable)
    }

    /// Set the window to [`DisplayMode::Windowed`]
    pub fn windowed(self) -> Self {
        self.display_mode(DisplayMode::Windowed)
    }

    /// Set the window to [`DisplayMode::Borderless`]
    pub fn borderless(self) -> Self {
        self.display_mode(DisplayMode::Borderless)
    }

    /// Set the adapter to use for rendering
    pub fn adapter(mut self, adapter: &'a Adapter) -> Self {
        self.adapter = Some(adapter);
        self
    }
}
