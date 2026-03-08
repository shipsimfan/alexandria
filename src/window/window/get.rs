use crate::{
    math::{Recti, Vector2i, Vector2u},
    window::Window,
};

impl<UserEvent: 'static + Send> Window<UserEvent> {
    /// Get the current title of the window
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn title_opt(&self) -> Option<String> {
        self.with_inner(|inner| inner.title().to_string())
    }

    /// Get the current title of the window
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn title(&self) -> String {
        self.title_opt().expect("window has been destroyed")
    }

    /// Get the position and size of the window's client area, in screen coordinates
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn rect_opt(&self) -> Option<Recti> {
        self.with_inner(|inner| inner.rect())
    }

    /// Get the position and size of the window's client area, in screen coordinates
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn rect(&self) -> Recti {
        self.rect_opt().expect("window has been destroyed")
    }

    /// Get the position of the top-left corner of the window's client area, in screen coordinates
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn position_opt(&self) -> Option<Vector2i> {
        self.rect_opt().map(|rect| rect.position)
    }

    /// Get the position of the top-left corner of the window's client area, in screen coordinates
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn position(&self) -> Vector2i {
        self.position_opt().expect("window has been destroyed")
    }

    /// Get the x-position of the top-left corner of the window's client area, in screen coordinates
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn x_opt(&self) -> Option<i32> {
        self.position_opt().map(|position| position.x)
    }

    /// Get the x-position of the top-left corner of the window's client area, in screen coordinates
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn x(&self) -> i32 {
        self.x_opt().expect("window has been destroyed")
    }

    /// Get the y-position of the top-left corner of the window's client area, in screen coordinates
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn y_opt(&self) -> Option<i32> {
        self.position_opt().map(|position| position.y)
    }

    /// Get the y-position of the top-left corner of the window's client area, in screen coordinates
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn y(&self) -> i32 {
        self.y_opt().expect("window has been destroyed")
    }

    /// Get the size of the window's client area
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn size_opt(&self) -> Option<Vector2i> {
        self.rect_opt().map(|rect| rect.size)
    }

    /// Get the size of the window's client area
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn size(&self) -> Vector2i {
        self.size_opt().expect("window has been destroyed")
    }

    /// Get the width of the window's client area
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn width_opt(&self) -> Option<i32> {
        self.size_opt().map(|size| size.x)
    }

    /// Get the width of the window's client area
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn width(&self) -> i32 {
        self.width_opt().expect("window has been destroyed")
    }

    /// Get the height of the window's client area
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn height_opt(&self) -> Option<i32> {
        self.size_opt().map(|size| size.y)
    }

    /// Get the height of the window's client area
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn height(&self) -> i32 {
        self.height_opt().expect("window has been destroyed")
    }

    /// Get the minimum size of the window's client area, in pixels
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn minimum_size_opt(&self) -> Option<Option<Vector2u>> {
        self.with_inner(|inner| inner.minimum_size())
    }

    /// Get the minimum size of the window's client area, in pixels
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn minimum_size(&self) -> Option<Vector2u> {
        self.minimum_size_opt().expect("window has been destroyed")
    }

    /// Get the maximum size of the window's client area, in pixels
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn maximum_size_opt(&self) -> Option<Option<Vector2u>> {
        self.with_inner(|inner| inner.maximum_size())
    }

    /// Get the maximum size of the window's client area, in pixels
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn maximum_size(&self) -> Option<Vector2u> {
        self.maximum_size_opt().expect("window has been destroyed")
    }

    /// Get the current content scale factor of the window
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn content_scale_opt(&self) -> Option<f32> {
        self.with_inner(|inner| inner.content_scale())
    }

    /// Get the current content scale factor of the window
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn content_scale(&self) -> f32 {
        self.content_scale_opt().expect("window has been destroyed")
    }

    /// Is the window currently in fullscreen mode?
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn is_fullscreen_opt(&self) -> Option<bool> {
        self.with_inner(|inner| inner.is_fullscreen())
    }

    /// Is the window currently in fullscreen mode?
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn is_fullscreen(&self) -> bool {
        self.is_fullscreen_opt().expect("window has been destroyed")
    }

    /// Is the window currently maximized?
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn is_maximized_opt(&self) -> Option<bool> {
        self.with_inner(|inner| inner.is_maximized())
    }

    /// Is the window currently maximized?
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn is_maximized(&self) -> bool {
        self.is_maximized_opt().expect("window has been destroyed")
    }

    /// Is the window currently minimized?
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn is_minimized_opt(&self) -> Option<bool> {
        self.with_inner(|inner| inner.is_minimized())
    }

    /// Is the window currently minimized?
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn is_minimized(&self) -> bool {
        self.is_minimized_opt().expect("window has been destroyed")
    }

    /// Is the window currently focused?
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn is_focused_opt(&self) -> Option<bool> {
        self.with_inner(|inner| inner.is_focused())
    }

    /// Is the window currently focused?
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn is_focused(&self) -> bool {
        self.is_focused_opt().expect("window has been destroyed")
    }

    /// Is the window currently visible?
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn is_visible_opt(&self) -> Option<bool> {
        self.with_inner(|inner| inner.is_visible())
    }

    /// Is the window currently visible?
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn is_visible(&self) -> bool {
        self.is_visible_opt().expect("window has been destroyed")
    }

    /// Is the window borderless?
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn is_borderless_opt(&self) -> Option<bool> {
        self.with_inner(|inner| inner.is_borderless())
    }

    /// Is the window borderless?
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn is_borderless(&self) -> bool {
        self.is_borderless_opt().expect("window has been destroyed")
    }

    /// Is the window resizable?
    ///
    /// Returns [`None`] if the window has been destroyed
    pub fn is_resizable_opt(&self) -> Option<bool> {
        self.with_inner(|inner| inner.is_resizable())
    }

    /// Is the window resizable?
    ///
    /// # Panics
    /// Panics if the window has been destroyed
    pub fn is_resizable(&self) -> bool {
        self.is_resizable_opt().expect("window has been destroyed")
    }
}
